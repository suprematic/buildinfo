pub mod v1 {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectInfo {
        pub name: String,
        pub version: String,
        pub as_string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RepoInfo {
        pub repository: String,
        pub reference: String,
        pub commit: String,

        #[serde(skip_serializing_if = "std::ops::Not::not")]
        pub dirty: bool,

        pub as_string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BuilderInfo {
        pub environment: String,
        pub timestamp: String,
        pub number: u32,
        pub trigger: String,
        pub as_string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BuildInfo {
        pub project: ProjectInfo,
        pub git: RepoInfo,
        pub build: BuilderInfo,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub properties: HashMap<String, String>,

        pub as_string: String,
    }
}

//pub use buildinfo_macro::buildinfo;
pub use v1::BuildInfo;
