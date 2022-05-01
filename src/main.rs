mod python_project;
use clap::{Parser, Subcommand};
use std::io;
use std::path::Path;

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

    author: String,
}

fn main() -> Result<(), io::Error> {
    let args: Args = Args::parse();

    match args.subcmd {
        SubCommand::New(t) => {
            let project_name = Path::new(&t.project_name);
            python_project::setup(project_name, &t.author)?
        }
    }

    Ok(())
}
