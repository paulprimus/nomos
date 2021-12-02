use std::path::PathBuf;

pub mod init;
pub mod error;


#[derive(Debug)]
pub enum RepoKind {
    Bare,
    Worktree
}

pub enum Path {
    Repo(PathBuf),
    Worktree(PathBuf)
}

