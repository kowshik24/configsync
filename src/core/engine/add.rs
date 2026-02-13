use crate::core::config::loader::ConfigLoader;
use crate::core::config::schema::{FileConfig, FileType};
use crate::core::fs::symlink;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn add<P: AsRef<Path>>(path: P) -> Result<()> {
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

    config.files.push(FileConfig {
        source,
        destination,
        file_type,
        platforms: vec!["*".to_string()],
        critical: false,
        protect: false,
    });

    ConfigLoader::save(&config, &config_path)?;
    println!("Added {:?} to config.", path);

    Ok(())
}
