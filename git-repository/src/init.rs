use std::fs;
use std::path::PathBuf;

use git_error::error::ErrorKind::{CreateDirectory};
use git_error::error::{ErrorKind, NomosError};
use crate::RepoKind;
use std::fs::OpenOptions;
use std::io::Write;

const GIT_DIR_NAME: &str = ".git";

/// Info Template
const TPL_INFO_EXCLUDE: &[u8] = include_bytes!("template/init/info/exclude");
/// Hooks Template
const TPL_HOOKS_APPLYPATCH_MSG: &[u8] = include_bytes!("template/init/hooks/applypatch-msg.sample");
const TPL_HOOKS_COMMIT_MSG: &[u8] = include_bytes!("template/init/hooks/commit-msg.sample");
const TPL_HOOKS_FSMONITOR_WATCHMAN: &[u8] = include_bytes!("template/init/hooks/fsmonitor-watchman.sample");
const TPL_HOOKS_POST_UPDATE: &[u8] = include_bytes!("template/init/hooks/post-update.sample");
const TPL_HOOKS_PRE_APPLYPATCH: &[u8] = include_bytes!("template/init/hooks/pre-applypatch.sample");
const TPL_HOOKS_PRE_COMMIT: &[u8] = include_bytes!("template/init/hooks/pre-commit.sample");
const TPL_HOOKS_PRE_MERGE_COMMIT: &[u8] = include_bytes!("template/init/hooks/pre-merge-commit.sample");
const TPL_HOOKS_PRE_PUSH: &[u8] = include_bytes!("template/init/hooks/pre-push.sample");
const TPL_HOOKS_PRE_REBASE: &[u8] = include_bytes!("template/init/hooks/pre-rebase.sample");
const TPL_HOOKS_PRE_RECEIVE: &[u8] = include_bytes!("template/init/hooks/pre-receive.sample");
const TPL_HOOKS_PREPARE_COMMIT_MSG: &[u8] = include_bytes!("template/init/hooks/prepare-commit-msg.sample");
const TPL_HOOKS_UPDATE: &[u8] = include_bytes!("template/init/hooks/update.sample");
const TPL_CONFIG: &[u8] = include_bytes!("template/init/config");
const TPL_DESCRIPTION: &[u8] = include_bytes!("template/init/description");
const TPL_HEAD: &[u8] = include_bytes!("template/init/HEAD");

pub fn init<T: Into<PathBuf>>(directory: T, kind: RepoKind) -> Result<(), NomosError> {
    let root = PathBuf::from(directory.into()).join(GIT_DIR_NAME);
    prepare_dir(&root)?;
    create_dir(&root)?;

    // info
    let info_dir = root.join("info");
    prepare_dir(&info_dir)?;
    create_dir(&info_dir);
    info_dir.join("exclude");
    write_file(TPL_INFO_EXCLUDE, &info_dir.join("exclude"))?;

    // hooks
    let mut hooks = root.join("hooks");
    create_dir(&hooks);
    for (tpl, filename) in [
        (TPL_HOOKS_UPDATE, "update.sample"),
        (TPL_HOOKS_PREPARE_COMMIT_MSG, "prepare-commit-msg.sample"),
        (TPL_HOOKS_PRE_RECEIVE, "pre-receive.sample"),
        (TPL_HOOKS_PRE_REBASE, "pre-rebase.sample"),
        (TPL_HOOKS_PRE_PUSH, "pre-push.sample"),
        (TPL_HOOKS_PRE_COMMIT, "pre-commit.sample"),
        (TPL_HOOKS_PRE_MERGE_COMMIT, "pre-merge-commit.sample"),
        (TPL_HOOKS_PRE_APPLYPATCH, "pre-applypatch.sample"),
        (TPL_HOOKS_POST_UPDATE, "post-update.sample"),
        (TPL_HOOKS_FSMONITOR_WATCHMAN, "fsmonitor-watchman.sample"),
        (TPL_HOOKS_COMMIT_MSG, "commit-msg.sample"),
        (TPL_HOOKS_APPLYPATCH_MSG, "applypatch-msg.sample"),
    ] {
        write_file(tpl, &hooks.join(filename))?;
    }

    // objects
    create_dir(&root.join("objects"))?;

    // refs
    create_dir(&root.join("refs"))?;

    // tags
    create_dir(&root.join("tags"))?;

    for (tpl, filename) in &[
        (TPL_HEAD, "HEAD"),
        (TPL_DESCRIPTION, "description"),
    ] {
            write_file(tpl, &root.join(filename))?;
    }

    let tpl = String::from_utf8(TPL_CONFIG.to_vec())?;
    let ntpl = tpl.replace("{bare-value}",
                match kind {
                    RepoKind::Bare => "true",
                    RepoKind::Worktree => "false",
                });
    write_file(ntpl.as_bytes(), &root.join("config"))?;

    Ok(())
}

/// check if .git directory is empty or not exiting
fn prepare_dir(git_dir: &PathBuf) -> Result<(), NomosError> {
    if git_dir.exists() {
        if std::fs::read_dir(git_dir)
            .map_err(|err| {
                NomosError::new(ErrorKind::IoOpen {
                    source: err,
                    path: git_dir.clone(),
                })
            })?
            .count()
            != 0
        {
            return Err(NomosError::new(ErrorKind::DirectoryNotEmpty { path: git_dir.clone() }));
            // return Err(ErrorKind::DirectoryNotEmpty { path: git_dir });
        }
    }
    Ok(())
}

fn create_dir(p: &PathBuf) -> Result<(), NomosError> {
    fs::create_dir_all(p).map_err(|e| {
        NomosError::new(CreateDirectory {
            source: e,
            path: p.clone(),
        })
    })
}

fn write_file(data: &[u8], path: &PathBuf) -> Result<(), NomosError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(path)
        .map_err(|e| {
            NomosError::new(ErrorKind::IoOpen {
                source: e,
                path: path.to_owned(),
            })
        })?;
    file.write_all(data).map_err(|e| {
        NomosError::new(ErrorKind::IoWrite {
            source: e,
            path: path.clone(),
        })
    })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::RepoKind;

    use super::init;

    #[test]
    fn test_init() {
        init("test_dir", RepoKind::Bare);
    }
}
