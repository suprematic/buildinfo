use std::env::var;

use anyhow::Result;

use crate::build::{BuildEnvironment, BuildInfo};

pub struct GitInfo {
    pub commit: String,
    pub reference: String,
    pub repository: String,
    pub dirty: bool,
    pub as_string: String,
}

fn as_string(commit: &str, dirty: bool, reference: &str) -> String{
    let short_commit = &commit[..7];
    let dirty = if dirty { "#dirty" } else { "" };
    format!("commit: {}{}, ref: {}", short_commit, dirty, reference)
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

    let as_string = as_string(&commit, dirty, &reference);

    Ok(GitInfo {
        commit,
        reference,
        repository,
        dirty,
        as_string,
    })
}

fn github_build_info() -> Result<GitInfo> {
    let commit = var("GITHUB_SHA")?;
    let reference = var("GITHUB_REF_NAME")?;
    let dirty = false;
    let as_string = as_string(&commit, dirty, &reference);

    Ok(GitInfo {
        commit,
        reference,
        repository: var("GITHUB_REPOSITORY")?,
        dirty,
        as_string,
    })
}

fn bitbucket_build_info() -> Result<GitInfo> {
    let commit = var("BITBUCKET_COMMIT")?;
    let reference = var("BITBUCKET_BRANCH").or_else(|_| var("BITBUCKET_TAG"))?;
    let dirty = false;
    let as_string = as_string(&commit, dirty, &reference);

    Ok(GitInfo {
        commit,
        reference,
        repository: var("BITBUCKET_REPO_FULL_NAME")?,
        dirty,
        as_string,
    })
}

fn codebuild_build_info() -> Result<GitInfo> {
    let commit = var("CODEBUILD_RESOLVED_SOURCE_VERSION")?;
    let reference = var("CODEBUILD_SOURCE_VERSION").unwrap_or_else(|_| "undefined".to_string());
    let dirty = false;
    let as_string = as_string(&commit, dirty, &reference);

    Ok(GitInfo {
        commit,
        reference,
        repository: var("CODEBUILD_SOURCE_REPO_URL")?,
        dirty,
        as_string,
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
