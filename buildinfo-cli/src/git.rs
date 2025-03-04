use std::env::var;

use anyhow::Result;

use crate::build::{BuildEnvironment, BuildInfo};

pub struct GitInfo {
    pub commit: String,
    pub reference: String,
    pub repository: String,
    pub dirty: bool,
}

fn local_build_info() -> Result<GitInfo> {
    let repo = git2::Repository::discover(".")?;
    let commit = repo.head()?.peel_to_commit()?.id().to_string();
    let reference = repo.head()?.shorthand().unwrap_or("default").to_string();
    let repository = repo.workdir().unwrap().to_string_lossy().to_string();

    let statuses = repo.statuses(None)?;

    let dirty = statuses
        .iter()
        .filter(|s| s.status() != git2::Status::IGNORED)
        .count()
        > 0;

    Ok(GitInfo {
        commit,
        reference,
        repository,
        dirty,
    })
}

fn github_build_info() -> Result<GitInfo> {
    Ok(GitInfo {
        commit: var("GITHUB_SHA")?,
        reference: var("GITHUB_REF_NAME")?,
        repository: var("GITHUB_REPOSITORY")?,
        dirty: false,
    })
}

fn bitbucket_build_info() -> Result<GitInfo> {
    Ok(GitInfo {
        commit: var("BITBUCKET_COMMIT")?,
        reference: var("BITBUCKET_BRANCH").or_else(|_| var("BITBUCKET_TAG"))?,
        repository: var("BITBUCKET_REPO_FULL_NAME")?,
        dirty: false,
    })
}

fn codebuild_build_info() -> Result<GitInfo> {
    Ok(GitInfo {
        commit: var("CODEBUILD_RESOLVED_SOURCE_VERSION")?,
        reference: var("CODEBUILD_SOURCE_VERSION").unwrap_or_else(|_| "undefined".to_string()),
        repository: var("CODEBUILD_SOURCE_REPO_URL")?,
        dirty: false,
    })
}

pub fn git_info(environment: &BuildInfo) -> Result<GitInfo> {
    match environment.environment {
        BuildEnvironment::Local => local_build_info(),
        BuildEnvironment::GitHub => github_build_info(),
        BuildEnvironment::BitBucket => bitbucket_build_info(),
        BuildEnvironment::CodeBuild => codebuild_build_info(),
    }
}
