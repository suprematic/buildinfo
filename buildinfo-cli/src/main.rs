use std::{
    env::{current_dir, set_current_dir},
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::ExitCode,
};

use anyhow::Result;

mod build;
mod cli;
mod git;
mod project;

use cli::CLI;

mod v1 {
    use super::build::BuildEnvironment;
    use buildinfo::v1::{BuildInfo, BuilderInfo, ProjectInfo, RepoInfo};
    use std::collections::HashMap;

    pub fn build(
        build_info: &super::build::BuildInfo,
        git_info: &super::git::GitInfo,
        project_info: &super::project::ProjectInfo,
    ) -> BuildInfo {
        BuildInfo {
            project: ProjectInfo {
                name: project_info.name.clone(),
                version: project_info.version.clone(),
                as_string: project_info.as_string.clone(),
            },

            git: RepoInfo {
                repository: git_info.repository.clone(),
                reference: git_info.reference.clone(),
                commit: git_info.commit.clone(),
                dirty: git_info.dirty,
                as_string: git_info.as_string.clone(),
            },

            build: BuilderInfo {
                timestamp: build_info.timestamp.clone(),
                number: build_info.number,
                trigger: build_info.trigger.clone(),
                environment: match build_info.environment {
                    BuildEnvironment::Local => "local",
                    BuildEnvironment::GitHub => "github",
                    BuildEnvironment::BitBucket => "bitbucket",
                    BuildEnvironment::CodeBuild => "codebuild",
                }
                .to_string(),
            },

            properties: HashMap::new(),
        }
    }
}

fn spit(content: &str, path: &Path) -> Result<()> {
    path.parent().map_or(Ok(()), create_dir_all)?;

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn generate() -> Result<()> {
    let mut paths = CLI.paths.clone();

    let build_info = build::build_info()?;
    let git_info = git::git_info(&build_info)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let cwd = current_dir()?;

    for path in paths {
        let mut target = cwd.clone();

        target.push(path);

        set_current_dir(&target)?;

        if CLI.verbose {
            println!("project directory '{}'", current_dir()?.display());
        }

        let project_info = project::project_info()?;

        let output = v1::build(&build_info, &git_info, &project_info);
        let output = serde_json::to_string_pretty(&output)?;

        if CLI.verbose {
            println!(
                "writing:\n{}\nto: '{}'",
                output,
                project_info.target_path.display()
            );
        }

        spit(&output, project_info.target_path.as_path())?;
    }

    Ok(())
}

fn main() -> ExitCode {
    generate().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        ExitCode::FAILURE
    })
}
