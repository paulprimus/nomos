use std::path::PathBuf;

pub mod init;

#[derive(Debug)]
pub enum RepoKind {
    Bare,
    Worktree
}

pub enum Path {
    Repo(PathBuf),
    Worktree(PathBuf)
}

