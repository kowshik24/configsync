use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo =
            Repository::init(path.as_ref()).context("Failed to initialize git repository")?;
        Ok(Self { repo })
    }

    pub fn clone<P: AsRef<Path>>(url: &str, path: P) -> Result<Self> {
        let repo =
            Repository::clone(url, path.as_ref()).context("Failed to clone git repository")?;
        Ok(Self { repo })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path.as_ref()).context("Failed to open git repository")?;
        Ok(Self { repo })
    }

    pub fn commit_all(&self, message: &str) -> Result<()> {
        let mut index = self.repo.index().context("Failed to open index")?;

        // Add all files (changes, new files, and deletions)
        index
            .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
            .context("Failed to add files to index")?;
        index.write().context("Failed to write index")?;

        let tree_id = index.write_tree().context("Failed to write tree")?;
        let tree = self
            .repo
            .find_tree(tree_id)
            .context("Failed to find tree")?;

        let signature = self.repo.signature().context("Failed to get signature")?;

        let parent_commit = match self.repo.head() {
            Ok(head) => {
                let target = head.target().context("Head has no target")?;
                Some(
                    self.repo
                        .find_commit(target)
                        .context("Failed to find head commit")?,
                )
            }
            Err(_) => None, // Initial commit
        };

        let parents = if let Some(ref p) = parent_commit {
            vec![p]
        } else {
            vec![]
        };

        self.repo
            .commit(
                Some("HEAD"), // Update HEAD
                &signature,
                &signature,
                message,
                &tree,
                &parents,
            )
            .context("Failed to commit")?;

        Ok(())
    }

    pub fn push(&self) -> Result<()> {
        let mut remote = self.repo.find_remote("origin").context(
            "Failed to find remote 'origin'. If this is a local-only repo, push is not needed.",
        )?;

        // We need to handle credentials here. For MVP, we'll try with default (ssh agent or credential helper).
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        let mut push_opts = git2::PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        // Get current branch name
        let head = self.repo.head().context("Failed to get HEAD")?;
        let branch_name = head.shorthand().unwrap_or("main");
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

        remote
            .push(&[&refspec], Some(&mut push_opts))
            .context("Failed to push to remote")?;

        Ok(())
    }

    pub fn pull(&self) -> Result<()> {
        let mut remote = self
            .repo
            .find_remote("origin")
            .context("Failed to find remote 'origin'.")?;

        // 1. Fetch
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        let mut fetch_opts = git2::FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        remote
            .fetch(&["main"], Some(&mut fetch_opts), None)
            .context("Failed to fetch")?;

        // 2. Merge (Fast-forward only for MVP simplicity)
        // Actually, we should look up local branch and merge upstream
        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        // Check analysis
        let (analysis, _preference) = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            println!("Already up to date.");
        } else if analysis.is_fast_forward() {
            let refname = "refs/heads/main"; // hardcoded for MVP
            let mut reference = self.repo.find_reference(refname)?;
            let name = reference.name().unwrap().to_string();
            let msg = format!(
                "Fast-Forward: Setting {} to id: {}",
                name,
                fetch_commit.id()
            );

            reference.set_target(fetch_commit.id(), &msg)?;
            self.repo.set_head(&name)?;
            self.repo
                .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            // Normal merge
            // For MVP, if it's not FF, we might just try to merge or error out.
            // Let's error out or auto-merge
            // Doing a real merge needs signature etc.
            // Let's rely on checkout for now?
            anyhow::bail!("Non fast-forward merge required. Manual intervention needed for now.");
        }

        Ok(())
    }

    pub fn log(&self) -> Result<()> {
        let mut revwalk = self.repo.revwalk().context("Failed to create revwalk")?;
        revwalk.push_head().context("Failed to push head")?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        println!("Commit History (Last 10):");
        for oid in revwalk.take(10) {
            let oid = oid.context("Failed to get oid")?;
            let commit = self
                .repo
                .find_commit(oid)
                .context("Failed to find commit")?;

            let short_id = &oid.to_string()[..7];
            let message = commit.summary().unwrap_or("<no message>");
            let author = commit.author();
            let name = author.name().unwrap_or("Unknown");

            let time = commit.time();
            let datetime = chrono::DateTime::from_timestamp(time.seconds(), 0)
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Unknown time".to_string());

            println!("{} {} - {} ({})", short_id, datetime, message, name);
        }
        Ok(())
    }

    pub fn revert(&self, commit_hash: Option<String>) -> Result<()> {
        // 1. Resolve commit to revert
        let commit = if let Some(hash) = commit_hash {
            let oid = git2::Oid::from_str(&hash).context("Invalid commit hash")?;
            self.repo.find_commit(oid).context("Commit not found")?
        } else {
            self.repo
                .head()?
                .peel_to_commit()
                .context("Failed to get HEAD commit")?
        };

        println!(
            "Reverting commit: {} - {}",
            commit.id(),
            commit.summary().unwrap_or("")
        );

        // 2. Perform Revert (in memory/index)
        // git2::revert modifies the index and working tree to reverse the commit
        let mut opts = git2::RevertOptions::new();
        self.repo
            .revert(&commit, Some(&mut opts))
            .context("Failed to revert")?;

        // 3. Commit the Revert
        let message = format!("Revert \"{}\"", commit.summary().unwrap_or(""));
        self.commit_all(&message)?;

        println!("Revert successful. New commit created.");
        Ok(())
    }
}
