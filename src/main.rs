use std::error::Error;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::io;
use clap::{Parser, Subcommand, Args};

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

fn exec(program: &str, args: Vec<&str>) -> Result<(), io::Error> {
    let r = Command::new(program)
        .args(args)
        .exec();
    Err(r)
}

fn handle_open_command(paths: Vec<String>, wait: bool) -> Result<(), Box<dyn Error>> {
    if paths.is_empty() {
        println!("No paths provided. Opening default editor.");
        exec("vim", vec![])?;
    } else {
        for path in paths {
            println!("Opening file: {}", path);
            exec("vim", vec![&path])?;
        }
    }
    if wait {
        println!("Waiting for editor to close...");
    }
    Ok(())
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
            println!("Setting default editor to {:?} (global: {})", editor, global);
        }
    }

    Ok(())
}
