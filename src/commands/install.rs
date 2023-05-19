use clap::Args;

#[derive(Debug, Args)]
pub struct CommandArgs {
    packages: Vec<String>,
    #[clap(short, long)]
    force: bool,
}

pub fn run(args: CommandArgs) {
    println!("here we will install packages");
    dbg!(args);
}
