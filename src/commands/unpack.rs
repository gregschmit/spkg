use clap::Args;

#[derive(Debug, Args)]
pub struct CommandArgs {
    #[clap(short, long)]
    path: String,
}

pub fn run(args: CommandArgs) {
    println!("here we will unpack a package");
    dbg!(args);
}
