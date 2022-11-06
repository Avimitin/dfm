use std::collections::HashMap;

use serde::{de::Deserializer, Deserialize};

/// Currently verified and tested usable distribution
#[derive(PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailableOS {
    ArchLinux,
    Ubuntu,
    Alpine,
}

/// List of dependencies name separated by os.
///
/// Some application have different nameing through the different packager in different distribution
pub type Dependencies = HashMap<AvailableOS, Vec<String>>;

/// Different types of sources, take different action for fetching them
#[derive(Debug)]
pub enum FetchType {
    Git(String),
    Remote(String),
    Local(std::path::PathBuf),
}

#[derive(Debug)]
pub struct Source {
    pub fetch_type: FetchType,
    pub alias: Option<String>,
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use std::path::PathBuf;
        use FetchType::*;

        let map: HashMap<String, String> = Deserialize::deserialize(d)?;

        let alias = map.get("alias").map(|s| s.to_string());

        if let Some(path) = map.get("path") {
            return Ok(Source {
                alias,
                fetch_type: Local(PathBuf::from(path)),
            });
        }

        if let Some(remote) = map.get("url") {
            return Ok(Source {
                alias,
                fetch_type: Remote(remote.to_string()),
            });
        }

        if let Some(git) = map.get("git") {
            return Ok(Source {
                alias,
                fetch_type: Git(git.to_string()),
            });
        }

        let lefted = map.keys().next();
        if lefted.is_none() {
            return Err(D::Error::custom("You should at least specify one key"));
        }

        Err(D::Error::custom(format!(
            "Unexpected key: {}",
            lefted.unwrap()
        )))
    }
}

/// Wrapper for file system operation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Mkdir(Vec<String>),
    Copy([String; 2]),
    SymLink([String; 2]),
    Remove(Vec<String>),
    Execute(Vec<String>),
}

/// Present a single step to be done
#[derive(Debug, Deserialize)]
pub struct Step {
    /// Descript the step, useful for logging
    pub name: Option<String>,
    /// List of action to be done
    pub actions: Vec<Action>,
    pub env: Option<HashMap<String, String>>,
}

/// Build time information for the dotfile
#[derive(Debug, Deserialize)]
pub struct DotFileInstallInfo {
    /// The name of the pending install configuration
    pub name: String,
    /// List of dependencies that pending downloaded by system packager
    pub depends: Dependencies,
    /// List of files to be fetched
    pub sources: Vec<Source>,
    /// List of action to be done
    pub steps: Vec<Step>,
}
