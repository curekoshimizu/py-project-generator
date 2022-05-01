use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    about = "Generate python project", 
    long_about = None
    )]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    New(NewCommand),
    // Update,
}

#[derive(Parser, Debug)]
struct NewCommand {
    project_name: String,
}

fn main() {
    let args: Args = Args::parse();

    match args.subcmd {
        SubCommand::New(t) => {
            dbg!(t.project_name);
        }
    }
}
