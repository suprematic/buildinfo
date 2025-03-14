use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum ProjectType {
    Rust,
    Java,
    JavaScript,
    Other,
}

pub struct ProjectInfo {
    #[allow(unused)]
    project_type: ProjectType,
    pub name: String,
    pub version: String,
    pub target_path: PathBuf,
}

fn project_type() -> ProjectType {
    fn exists(path: &str) -> bool {
        std::fs::exists(path).unwrap_or(false)
    }

    if exists("Cargo.toml") {
        ProjectType::Rust
    } else if exists("pom.xml") {
        ProjectType::Java
    } else if exists("package.json") {
        ProjectType::JavaScript
    } else {
        ProjectType::Other
    }
}

fn rust_project_info() -> Result<ProjectInfo> {
    let cargo_toml = slurp::read_all_to_string("Cargo.toml")?;
    let cargo_toml = toml::from_str::<toml::Table>(&cargo_toml)?;

    let package = &cargo_toml["package"];
    let name = package["name"].as_str().unwrap_or("unknown").to_string();
    let version = package["version"].as_str().unwrap_or("unknown").to_string();

    let target_path = current_dir()?.join("src/buildinfo.json");

    Ok(ProjectInfo {
        project_type: ProjectType::Rust,
        name,
        version,
        target_path,
    })
}

fn java_project_info() -> Result<ProjectInfo> {
    #[derive(Deserialize, Debug)]
    struct Root {
        #[serde(rename = "groupId")]
        group_id: String,
        #[serde(rename = "artifactId")]
        artifact_id: String,
        #[serde(rename = "version")]
        version: String,
    }

    let pom_xml = slurp::read_all_to_string("pom.xml")?;
    let pom_xml = fast_xml::de::from_str::<Root>(&pom_xml)?;

    let target_path = current_dir()?.join("src/main/resources/META-INF/buildinfo.json");

    Ok(ProjectInfo {
        project_type: ProjectType::Java,
        name: format!("{}/{}", pom_xml.group_id, pom_xml.artifact_id),
        version: pom_xml.version,
        target_path,
    })
}

fn javascript_project_info() -> Result<ProjectInfo> {
    let project_json = slurp::read_all_to_string("package.json")?;
    let project_json = serde_json::from_str::<serde_json::Value>(&project_json)?;

    let name = project_json["name"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let version = project_json["version"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let target_path = current_dir()?.join("src/buildinfo.json");

    Ok(ProjectInfo {
        project_type: ProjectType::JavaScript,
        name,
        version,
        target_path,
    })
}

pub fn project_info() -> Result<ProjectInfo> {
    match project_type() {
        ProjectType::Rust => rust_project_info(),
        ProjectType::Java => java_project_info(),
        ProjectType::JavaScript => javascript_project_info(),
        ProjectType::Other => Ok(ProjectInfo {
            project_type: ProjectType::Other,
            name: "unknown".to_string(),
            version: "unknown".to_string(),
            target_path: current_dir()?.join("buildinfo.json"),
        }),
    }
}
