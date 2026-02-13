use anyhow::Context;
use std::fs;
use std::path::Path;

pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> anyhow::Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    if link.exists() {
        // For MVP, we'll just error if it exists.
        // Later we can implement backup or force overwrite.
        anyhow::bail!("Destination {:?} already exists", link);
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = link.parent() {
        fs::create_dir_all(parent).context("Failed to create parent directory for symlink")?;
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(original, link).context(format!(
            "Failed to create symlink from {:?} to {:?}",
            original, link
        ))?;
    }

    #[cfg(windows)]
    {
        // TODO: Implement Windows support using junction or symlink_file
        anyhow::bail!("Windows symlink support not yet implemented in this MVP phase");
    }

    Ok(())
}
