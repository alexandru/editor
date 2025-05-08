use clap::{Args, Parser, Subcommand};
use std::error::Error;
use crate::editor::osutils::handle_open_command;

pub mod editor;

#[derive(Parser, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
#[command(name = "editor")]
#[command(about = "CLI utility for invoking your favorite editor.", long_about = None)]
struct Cli {
    #[clap(flatten)]
    open: OpenArgs,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args, Debug)]
struct OpenArgs {
    #[arg(required = false)]
    paths: Vec<String>,

    #[arg(long, default_value_t = false)]
    wait: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Open(OpenArgs),
    SetDefault {
        editor: Editor,

        #[arg(long, default_value_t = false)]
        global: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Editor {
    Code,
    Codium,
    Zed,
    Idea,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Open(open_args)) => {
            handle_open_command(open_args.paths, open_args.wait)?;
        }
        None => {
            handle_open_command(cli.open.paths, cli.open.wait)?;
        }
        Some(Commands::SetDefault { editor, global }) => {
            println!(
                "Setting default editor to {:?} (global: {})",
                editor, global
            );
        }
    }

    Ok(())
}
