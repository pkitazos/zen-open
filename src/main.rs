use crate::error::{Error, Result};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

mod error;
mod profiles;

/// Opening and switching between Zen instances
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Profile name
    name: Option<String>, // ! this can't be required!

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

#[derive(Subcommand, Debug)]
enum Commands {
    /// lists available profiles
    List {
        /// formats output as json
        #[arg(long)]
        json: bool,
    },
}

struct App {
    app: PathBuf,
    profiles_dir: PathBuf,
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

fn build_app_config(
    maybe_app_path: Option<String>,
    maybe_profiles_dir_path: Option<String>,
) -> Result<App> {
    let app = match maybe_app_path {
        Some(s) => PathBuf::from(s),
        // todo: make this OS agnostict
        None => PathBuf::from("/Applications/Zen.app"),
    };

    let profiles_dir = match maybe_profiles_dir_path {
        Some(s) => PathBuf::from(s),
        None => PathBuf::from("/Users/petroskitazos/Library/Application Support/zen"), // dirs::config_dir().ok_or_else(|| Error::Io(todo!()))?,
    };

    Ok(App { app, profiles_dir })
}

fn run() -> Result<()> {
    let args = Args::parse();

    let app = build_app_config(args.app, args.profiles_dir)?;

    match args.command {
        Some(Commands::List { .. }) => {
            let (profiles, default_idx) = profiles::load(&app.profiles_dir)?;
            let ps = profiles
                .into_iter()
                .enumerate()
                .map(|(i, p)| {
                    if default_idx.is_some_and(|idx| idx == i) {
                        format!("* {}", p.name)
                    } else {
                        format!("  {}", p.name)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            println!("{ps}")
        }
        None => {
            // [ ] match on args.name
            // [ ] find profile
            // [ ] report
            match &args.url {
                Some(url) => println!("Opening {} in {:?}", url, args.name),
                None => println!("Opening {:?}", args.name),
            }
        }
    }
    Ok(())
}
