use crate::core::config::loader::ConfigLoader;
use crate::core::config::schema::{FileConfig, FileType};
use crate::core::fs::symlink;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::Path;

pub fn add<P: AsRef<Path>>(path: P, role: Vec<String>) -> Result<()> {
    let path = path
        .as_ref()
        .canonicalize()
        .context("Failed to resolve path")?;

    // 1. Locate repo/config
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("team-config.toml");

    if !config_path.exists() {
        anyhow::bail!("ConfigSync not initialized. Run `configsync init` first.");
    }

    // 2. Load config
    let mut config = ConfigLoader::load(&config_path)?;

    // 3. Determine relative path for repo storage
    // For now, we just use the filename in the root of the repo, or maybe mirror structure?
    // Let's mirror the structure relative to home directory if possible, or just flatten?
    // Architecture doc says: `source = "nvim"`, `destination = "~/.config/nvim"`.
    // Let's simplify: if adding `~/.config/nvim/init.lua`, store as `nvim/init.lua` in repo?
    // Or just ask user?
    // For MVP, let's just use the basename. If collision, error.
    let file_name = path.file_name().context("Invalid path")?;
    let repo_path = config_dir.join(file_name);

    if repo_path.exists() {
        anyhow::bail!("File {:?} already exists in repository", file_name);
    }

    // 4. Move file to repo
    println!("Moving {:?} to {:?}", path, repo_path);
    if path.is_dir() {
        // recursive copy/move? Or just rename?
        fs::rename(&path, &repo_path).context("Failed to move directory")?;
    } else {
        fs::rename(&path, &repo_path).context("Failed to move file")?;
    }

    // 5. Create symlink back
    println!("Creating symlink from {:?} to {:?}", repo_path, path);
    symlink::create_symlink(&repo_path, &path)?;

    // 6. Update config
    let source = file_name.to_string_lossy().to_string();
    let destination = path.to_string_lossy().to_string(); // This is absolute path.
                                                          // Ideally we'd make it relative to HOME if possible for portability.
                                                          // For MVP, absolute is okay-ish, or try to replace HOME with ~

    let file_type = if repo_path.is_dir() {
        FileType::Directory
    } else {
        FileType::File
    };

    let roles = if role.is_empty() { None } else { Some(role) };

    config.files.push(FileConfig {
        source,
        destination,
        file_type,
        platforms: vec!["*".to_string()],
        critical: false,
        protect: false,
        roles,
    });

    ConfigLoader::save(&config, &config_path)?;
    println!("Added {:?} to config.", path);

    Ok(())
}

pub fn add_secret<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path
        .as_ref()
        .canonicalize()
        .context("Failed to resolve path")?;

    // 1. Locate repo/config
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("team-config.toml");

    if !config_path.exists() {
        anyhow::bail!("ConfigSync not initialized. Run `configsync init` first.");
    }

    // 2. Load config
    let mut config = ConfigLoader::load(&config_path)?;

    // 3. Load Keys
    let identity = crate::core::secret::keys::load_key()
        .context("Failed to load secret key. Have you run `configsync secrets init`?")?;
    let public_key = crate::core::secret::keys::get_public_key(&identity);

    // 4. Encrypt
    println!("Reading {:?}", path);
    let content = fs::read(&path).context("Failed to read secret file")?;
    println!("Encrypting...");
    let encrypted = crate::core::secret::cipher::encrypt(&content, &public_key)?;

    // 5. Save to Repo
    let file_name = path.file_name().context("Invalid path")?;
    let secret_dir = config_dir.join("secrets");
    fs::create_dir_all(&secret_dir)?;

    let encrypted_filename = format!("{}.age", file_name.to_string_lossy());
    let repo_path = secret_dir.join(&encrypted_filename);

    println!("Saving encrypted file to {:?}", repo_path);
    fs::write(&repo_path, encrypted).context("Failed to write encrypted file")?;

    // 6. Update Config
    let source = format!("secrets/{}", encrypted_filename);
    let destination = path.to_string_lossy().to_string();

    // Check if already exists
    if config.files.iter().any(|f| f.destination == destination) {
        println!("File already tracked. Updating encrypted content only.");
        // We already wrote the file, so we are good.
        // We might want to ensure the type is set to Secret if it wasn't.
    } else {
        config.files.push(FileConfig {
            source,
            destination,
            file_type: FileType::Secret,
            platforms: vec!["*".to_string()],
            critical: false,
            protect: false,
            roles: None, // Secrets are usually machine-specific in this MVP personal-sync model, or we can add roles later
        });
        ConfigLoader::save(&config, &config_path)?;
        println!("Added secret {:?} to config.", path);
    }

    Ok(())
}
