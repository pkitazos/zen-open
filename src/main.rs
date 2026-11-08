use clap::{Parser, Subcommand};
use std::process::ExitCode;

mod error;

/// Opening and switching between Zen instances
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Profile name
    name: String,

    /// URL to open (optional)
    url: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the Zen application bundle
    #[arg(long, name = "path")]
    app: Option<String>,

    /// Path to the folder containing profiles.ini
    #[arg(long, name = "path2")] // <- todo: better name
    profiles_dir: Option<String>,

    /// Print the actions and argv that would run, without running them
    #[arg(long)]
    dry_run: bool,

    /// Print diagnostics to stderr
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// lists available profiles
    List {
        /// formats output as json
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("zen-open: {e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn run() -> error::Result<()> {
    let args = Args::parse();
    match &args.url {
        Some(url) => println!("Opening {} in {}", url, args.name),
        None => println!("Opening {}", args.name),
    }
    Ok(())
}
