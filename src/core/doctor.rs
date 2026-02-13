use crate::core::config::loader::ConfigLoader;
use crate::core::config::schema::FileType;
use crate::core::state::LocalState;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;

pub fn check() -> Result<()> {
    println!("🩺 ConfigSync Doctor\n");
    let mut issues_found = false;

    // 1. Check Config Directory & File
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("team-config.toml");

    if !config_dir.exists() {
        println!("❌ Config directory missing: {:?}", config_dir);
        return Ok(());
    } else {
        println!("✅ Config directory exists: {:?}", config_dir);
    }

    if !config_path.exists() {
        println!("❌ Config file missing: {:?}", config_path);
        return Ok(());
    } else {
        println!("✅ Config file exists");
    }

    // 2. Parsability Check
    let config = match ConfigLoader::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            println!("❌ Config file invalid: {}", e);
            return Ok(());
        }
    };
    println!("✅ Config file passes validation");

    // 3. Git Repo Status
    // We can use our GitRepository wrapper or direct git2 check.
    // Let's use GitRepository::open if possible, or build a simple check here.
    match crate::core::git::repository::GitRepository::open(config_dir) {
        Ok(_) => println!("✅ Git repository valid"),
        Err(e) => {
            println!("❌ Git repository issue: {}", e);
            issues_found = true;
        }
    }

    // 4. File Symlink Checks
    println!("\nChecking {} tracked files...", config.files.len());
    let state = LocalState::load().unwrap_or_default();

    for file in &config.files {
        // Role check
        if let Some(ref required_roles) = file.roles {
            if !required_roles.is_empty() {
                let has_role = required_roles.iter().any(|r| state.has_role(r));
                if !has_role {
                    // Skip checking files not meant for this machine role, unless we want to be strict?
                    // Skipping is better, otherwise Doctor complains about missing files intended for other roles.
                    continue;
                }
            }
        }

        let source_path = config_dir.join(&file.source);
        let expanded_dest = shellexpand::tilde(&file.destination);
        let dest_path = std::path::PathBuf::from(expanded_dest.into_owned());

        if !source_path.exists() {
            println!("❌ Source missing in repo: {:?}", source_path);
            issues_found = true;
            continue;
        }

        if !dest_path.exists() {
            println!("❌ Destination missing: {:?}", dest_path);
            issues_found = true;
            continue;
        }

        // Check if it's a symlink (or junction on Windows) or file type mismatch
        // For Secret types, it's a file copy, not a symlink.
        match file.file_type {
            FileType::Secret => {
                // Secrets are files, not symlinks.
                if !dest_path.is_file() {
                    println!("⚠️ Destination {:?} should be a file (Secret)", dest_path);
                    issues_found = true;
                } else {
                    // Maybe check if content matches decrypted? Too expensive/risky for doctor?
                    // Just existence is okay for now.
                }
            }
            _ => {
                // Should be a symlink
                match fs::symlink_metadata(&dest_path) {
                    Ok(metadata) => {
                        if !metadata.file_type().is_symlink() {
                            println!(
                                "❌ Destination {:?} is NOT a symlink (Expected symlink)",
                                dest_path
                            );
                            issues_found = true;
                        } else {
                            // Check target
                            match fs::read_link(&dest_path) {
                                Ok(target) => {
                                    if target != source_path {
                                        println!(
                                            "⚠️ Symlink {:?} points to {:?}, expected {:?}",
                                            dest_path, target, source_path
                                        );
                                        // This might be okay if it's relative?
                                        // Canonicalize both?
                                        if let (Ok(p1), Ok(p2)) = (
                                            fs::canonicalize(&target),
                                            fs::canonicalize(&source_path),
                                        ) {
                                            if p1 != p2 {
                                                issues_found = true;
                                            }
                                        } else {
                                            issues_found = true;
                                        }
                                    }
                                }
                                Err(_) => {
                                    println!("❌ Failed to read link target for {:?}", dest_path);
                                    issues_found = true;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("❌ Failed to read metadata for {:?}", dest_path);
                        issues_found = true;
                    }
                }
            }
        }
    }

    // 5. Secrets Key Check
    // If there are any secrets in config, we should check for key.
    let has_secrets = config
        .files
        .iter()
        .any(|f| matches!(f.file_type, FileType::Secret));
    if has_secrets {
        println!("\nChecking Secrets Key...");
        if let Ok(path) = crate::core::secret::keys::get_key_path() {
            if path.exists() {
                println!("✅ Key file exists: {:?}", path);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let perms = fs::metadata(&path)?.permissions();
                    if perms.mode() & 0o777 != 0o600 {
                        println!(
                            "⚠️ Key file permissions unsafe: {:o} (Should be 600)",
                            perms.mode() & 0o777
                        );
                        issues_found = true;
                    } else {
                        println!("✅ Key file permissions safe (600)");
                    }
                }
            } else {
                println!("❌ Key file missing! Secrets cannot be decrypted.");
                issues_found = true;
            }
        }
    }

    println!("\n----------------------------------------");
    if issues_found {
        println!("⚠️ Issues found. Please review the output above.");
        // Suggest fixes?
        println!("Try running `configsync apply` to fix missing links/files.");
    } else {
        println!("✅ All systems operational.");
    }

    Ok(())
}
