use std::collections::HashMap;

use serde::Deserialize;

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
pub type Source = HashMap<String, String>;

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
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
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
    pub steps: Vec<Step>
}
