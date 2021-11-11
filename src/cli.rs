use clap::{AppSettings, Parser};

#[derive(Debug, Parser)]
#[clap(version="0.1", author="Paul Primus <paul.primus@gmail.com>")]
pub struct Args {

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Init,
}

// pub fn run() {
//     let args = Args::parse();
//     println!("{:?}", args);
// }