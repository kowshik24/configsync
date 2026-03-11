use crate::core::config::loader::ConfigLoader;
use crate::core::fs::symlink;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use shellexpand;
use std::fs;
use std::path::PathBuf;

pub fn apply() -> Result<()> {
    // 1. Locate config
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("team-config.toml");

    if !config_path.exists() {
        println!("No team-config.toml found. Nothing to apply.");
        return Ok(());
    }

    // 2. Load config
    let config = ConfigLoader::load(&config_path)?;

    println!("Applying configurations for team: {}", config.team.name);

    // 2a. Load local state for roles
    use crate::core::state::LocalState;
    let state = LocalState::load().unwrap_or_default();
    println!("Current machine roles: {:?}", state.roles);

    // 3. Iterate files and symlink
    for file in config.files {
        // Role check
        if let Some(ref required_roles) = file.roles {
            if !required_roles.is_empty() {
                let has_role = required_roles.iter().any(|r| state.has_role(r));
                if !has_role {
                    println!(
                        "Skipping {:?} (required roles: {:?})",
                        file.source, required_roles
                    );
                    continue;
                }
            }
        }

        let source_path = config_dir.join(&file.source);

        let expanded_dest = shellexpand::tilde(&file.destination);
        let dest_path = PathBuf::from(expanded_dest.into_owned());

        if !source_path.exists() {
            println!(
                "Warning: Source file {:?} does not exist. Skipping.",
                source_path
            );
            continue;
        }

        use crate::core::config::schema::FileType;
        match file.file_type {
            FileType::Secret => {
                println!("Decrypting secret {:?} -> {:?}", source_path, dest_path);
                // Load key (lazy load? for now just load every time or load once outside loop)
                // Let's load once outside loop if possible, or just here.
                let identity_result = crate::core::secret::keys::load_key();

                if let Ok(identity) = identity_result {
                    let encrypted_content =
                        std::fs::read(&source_path).context("Failed to read encrypted file")?;
                    match crate::core::secret::cipher::decrypt(&encrypted_content, &identity) {
                        Ok(decrypted) => {
                            if let Some(parent) = dest_path.parent() {
                                std::fs::create_dir_all(parent)?;
                            }
                            std::fs::write(&dest_path, decrypted)?;
                            #[cfg(unix)]
                            {
                                // Secrets should be 600
                                use std::os::unix::fs::PermissionsExt;
                                let mut perms = std::fs::metadata(&dest_path)?.permissions();
                                perms.set_mode(0o600);
                                std::fs::set_permissions(&dest_path, perms)?;
                            }
                            println!("Restored secret.");
                        }
                        Err(e) => println!("Failed to decrypt: {}", e),
                    }
                } else {
                    println!("Skipping secret: No private key found. Run `configsync secrets init` or restore key.");
                }
            }
            _ => {
                println!("Linking {:?} <- {:?}", dest_path, source_path);

                // If destination already points to the expected source, treat it as healthy.
                match fs::symlink_metadata(&dest_path) {
                    Ok(metadata) if metadata.file_type().is_symlink() => {
                        match fs::read_link(&dest_path) {
                            Ok(link_target) => {
                                let resolved_target = if link_target.is_absolute() {
                                    link_target
                                } else if let Some(parent) = dest_path.parent() {
                                    parent.join(link_target)
                                } else {
                                    link_target
                                };

                                let same_target = fs::canonicalize(&resolved_target)
                                    .ok()
                                    .zip(fs::canonicalize(&source_path).ok())
                                    .map(|(a, b)| a == b)
                                    .unwrap_or(false);

                                if same_target {
                                    println!("Already linked. Skipping.");
                                    continue;
                                }

                                println!(
                                    "Failed: Destination exists and points elsewhere ({:?}).",
                                    resolved_target
                                );
                                continue;
                            }
                            Err(e) => {
                                println!("Failed: Could not read existing symlink: {}", e);
                                continue;
                            }
                        }
                    }
                    Ok(_) => {
                        println!("Failed: Destination exists and is not a symlink.");
                        continue;
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            println!("Failed: Could not inspect destination: {}", e);
                            continue;
                        }
                        // Destination does not exist; create symlink below.
                    }
                }

                match symlink::create_symlink(&source_path, &dest_path) {
                    Ok(_) => println!("OK"),
                    Err(e) => println!("Failed: {}", e),
                }
            }
        }
    }

    Ok(())
}
