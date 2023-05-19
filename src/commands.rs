mod install;
mod list;
mod pack;
mod unpack;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Install(install::CommandArgs),
    List(list::CommandArgs),
    Pack(pack::CommandArgs),
    Unpack(unpack::CommandArgs),
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

pub fn dispatch(args: Args) {
    match args.command {
        Commands::List(args) => list::run(args),
        Commands::Install(args) => install::run(args),
        Commands::Pack(args) => pack::run(args),
        Commands::Unpack(args) => unpack::run(args),
    }
}
