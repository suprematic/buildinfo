use std::env::var;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum BuildEnvironment {
    GitHub,
    BitBucket,
    CodeBuild,
    Local,
}

pub struct BuildInfo {
    pub environment: BuildEnvironment,
    pub trigger: String,
    pub number: u32,
    pub timestamp: String,
    pub as_string: String,
}

fn build_environment() -> BuildEnvironment {
    if var("GITHUB_RUN_NUMBER").is_ok() {
        BuildEnvironment::GitHub
    } else if var("BITBUCKET_BUILD_NUMBER").is_ok() {
        BuildEnvironment::BitBucket
    } else if var("CODEBUILD_BUILD_ID").is_ok() {
        BuildEnvironment::CodeBuild
    } else {
        BuildEnvironment::Local
    }
}

fn timestamp() -> String {
    let timestamp: std::sync::LazyLock<chrono::DateTime<chrono::Utc>> =
        std::sync::LazyLock::new(|| chrono::Utc::now());

    timestamp.to_rfc3339()
}

fn as_string(number: u32, timestamp: &str, trigger: &str) -> String {
    format!("build #{} at {} by {}", number, timestamp, trigger)
}

fn github_build_info() -> Result<BuildInfo> {
    let trigger = var("GITHUB_ACTOR")?;
    let number = var("GITHUB_RUN_NUMBER")?.parse()?;
    let timestamp = timestamp();
    let as_string = as_string(number, &timestamp, &trigger);

    Ok(BuildInfo {
        environment: BuildEnvironment::GitHub,
        trigger,
        number,
        timestamp,
        as_string,
    })
}

fn bitbucket_build_info() -> Result<BuildInfo> {
    let trigger = var("BITBUCKET_BUILD_CREATOR")?;
    let number = var("BITBUCKET_BUILD_NUMBER")?.parse()?;
    let timestamp = var("BITBUCKET_BUILD_CREATED_ON")?;
    let as_string = as_string(number, &timestamp, &trigger);

    Ok(BuildInfo {
        environment: BuildEnvironment::BitBucket,
        trigger,
        number,
        timestamp,
        as_string,
    })
}

fn codebuild_build_info() -> Result<BuildInfo> {
    let trigger = var("CODEBUILD_INITIATOR")?;
    let number = var("CODEBUILD_BUILD_NUMBER")?.parse()?;
    let timestamp = var("CODEBUILD_START_TIME")?;
    let as_string = as_string(number, &timestamp, &trigger);

    Ok(BuildInfo {
        environment: BuildEnvironment::CodeBuild,
        trigger,
        number,
        timestamp,
        as_string,
    })
}

fn local_build_environment() -> Result<BuildInfo> {
    let trigger = whoami::username();
    let number = 1;
    let timestamp = timestamp();
    let as_string = as_string(number, &timestamp, &trigger);

    Ok(BuildInfo {
        environment: BuildEnvironment::Local,
        trigger,
        number,
        timestamp,
        as_string,
    })
}

pub fn build_info() -> Result<BuildInfo> {
    match build_environment() {
        BuildEnvironment::Local => local_build_environment(),
        BuildEnvironment::GitHub => github_build_info(),
        BuildEnvironment::BitBucket => bitbucket_build_info(),
        BuildEnvironment::CodeBuild => codebuild_build_info(),
    }
}
