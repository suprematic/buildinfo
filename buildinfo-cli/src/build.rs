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

fn github_build_info() -> Result<BuildInfo> {
    Ok(BuildInfo {
        environment: BuildEnvironment::GitHub,
        trigger: var("GITHUB_ACTOR")?,
        number: var("GITHUB_RUN_NUMBER")?.parse()?,
        timestamp: var("GITHUB_RUN_AT")?,
    })
}

fn bitbucket_build_info() -> Result<BuildInfo> {
    Ok(BuildInfo {
        environment: BuildEnvironment::BitBucket,
        trigger: var("BITBUCKET_BUILD_CREATOR")?,
        number: var("BITBUCKET_BUILD_NUMBER")?.parse()?,
        timestamp: var("BITBUCKET_BUILD_CREATED_ON")?,
    })
}

fn codebuild_build_info() -> Result<BuildInfo> {
    Ok(BuildInfo {
        environment: BuildEnvironment::CodeBuild,
        trigger: var("CODEBUILD_INITIATOR")?,
        number: var("CODEBUILD_BUILD_NUMBER")?.parse()?,
        timestamp: var("CODEBUILD_START_TIME")?,
    })
}

fn local_build_environment() -> Result<BuildInfo> {
    static TIMESTAMP: std::sync::LazyLock<chrono::DateTime<chrono::Local>> =
        std::sync::LazyLock::new(|| chrono::Local::now());

    Ok(BuildInfo {
        environment: BuildEnvironment::Local,
        trigger: whoami::username(),
        number: 1,
        timestamp: TIMESTAMP.to_rfc3339(),
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
