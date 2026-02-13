use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn get_key_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    // Store in data_local_dir (e.g., ~/.local/share/configsync/key.txt)
    // This is explicitly NOT the config dir which might be git-tracked.
    Ok(proj_dirs.data_local_dir().join("key.txt"))
}

pub fn generate_key() -> Result<String> {
    use secrecy::ExposeSecret;
    let key = age::x25519::Identity::generate();
    Ok(key.to_string().expose_secret().trim().to_string())
}

pub fn save_key(key: &str) -> Result<()> {
    let path = get_key_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Set permissions to 600 (Unix only)
    #[cfg(unix)]
    {
        // checking permissions on the parent dir would be good too, but let's focus on file
    }

    fs::write(&path, key).context("Failed to write key file")?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms)?;
    }

    Ok(())
}

pub fn load_key() -> Result<age::x25519::Identity> {
    let path = get_key_path()?;
    if !path.exists() {
        anyhow::bail!("No key found. Run `configsync secrets init` first.");
    }
    let content = fs::read_to_string(path)?;
    let key = content.trim().parse::<age::x25519::Identity>()
        .map_err(|e| anyhow::anyhow!("Failed to parse key: {}", e))?;
    Ok(key)
}

pub fn get_public_key(identity: &age::x25519::Identity) -> age::x25519::Recipient {
    identity.to_public()
}
