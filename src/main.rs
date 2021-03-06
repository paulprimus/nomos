use clap::Parser;
use git_repository;
use git_error::error::NomosError;
use git_repository::RepoKind;
use crate::cli::{Args, SubCommand};

mod cli;

fn main() -> Result<(), NomosError> {
    let args = Args::parse();
    match args.cmd {
        SubCommand::Init => git_repository::init::init("../test3", RepoKind::Worktree),
    }
}
