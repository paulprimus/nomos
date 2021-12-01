use clap::Parser;
use git_repository;
use crate::cli::{Args, SubCommand};

mod cli;

fn main() -> Result<(), NomosError> {
    let args = Args::parse();
    match args.cmd {
        SubCommand::Init => git_repository::init::init("../test3", RepoKind::Worktree),
    }
}
