use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamConfig {
    pub team: Team,
    pub repository: Repository,
    #[serde(default)]
    pub files: Vec<FileConfig>,
    #[serde(default)]
    pub secrets: SecretsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub maintainers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub url: String,
    pub branch: String,
    #[serde(default = "default_auto_update_interval")]
    pub auto_update_interval: u64,
}

fn default_auto_update_interval() -> u64 {
    300
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub source: String,
    pub destination: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub critical: bool,
    #[serde(default)]
    pub protect: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
    Directory,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SecretsConfig {
    pub vault_enabled: bool,
    pub vault_type: String,
    #[serde(default)]
    pub encrypted_files: Vec<String>,
}

impl Default for TeamConfig {
    fn default() -> Self {
        TeamConfig {
            team: Team {
                name: "default-team".to_string(),
                maintainers: vec![],
            },
            repository: Repository {
                url: "".to_string(),
                branch: "main".to_string(),
                auto_update_interval: 300,
            },
            files: vec![],
            secrets: SecretsConfig::default(),
        }
    }
}
