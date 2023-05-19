use clap::Args;

/// List packages.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[clap(short, long)]
    all: bool,
}

pub fn run(args: CommandArgs) {
    println!("here we will list packages");
    dbg!(args);
}
