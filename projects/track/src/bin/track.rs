use std::path::PathBuf;

use clap::{Parser, Subcommand};
use error_stack::{Report, Result, ResultExt};
use track::{
    error::Suggestion,
    init::{self, ENV_FILTER_TARGETS},
    tracker::FlatFileTracker,
    AppError, TimeTracker,
};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    /// Path to database file
    #[arg(short = 'd', long)]
    db_dir: Option<PathBuf>,

    /// Path to lockfile
    #[arg(short = 'l', long)]
    lockfile: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start tracking time
    Start,

    /// Stop tracking time
    Stop,
}

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing(&ENV_FILTER_TARGETS);

    let args = Cli::parse();

    let db_dir = match args.db_dir {
        Some(db_dir) => db_dir,
        None => {
            let mut db_dir = dirs::data_dir()
                .ok_or(AppError)
                .attach_printable("failed to discover data directory")
                .attach(Suggestion("use the -d flag to specify a database path"))?;
            db_dir.push("track");
            std::fs::create_dir_all(&db_dir)
                .change_context(AppError)
                .attach_printable("failed to create track db directory")?;
            db_dir.push("records.json");
            db_dir
        }
    };
    let lockfile = match args.lockfile {
        Some(lockfile) => lockfile,
        None => {
            let mut lockfile = dirs::cache_dir()
                .ok_or(AppError)
                .attach_printable("failed to discover cache directory")
                .attach(Suggestion("use the -l flag to specify a lockfile path"))?;
            lockfile.push("track");
            std::fs::create_dir_all(&lockfile)
                .change_context(AppError)
                .attach_printable("failed to create track cache directory")?;
            lockfile.push("track.lock");
            lockfile
        }
    };

    let mut tracker = FlatFileTracker::new(db_dir, lockfile)
        .change_context(AppError)
        .attach_printable("failed to initialize backend")?;

    match args.command {
        Command::Start => {
            if tracker
                .is_tracking()
                .change_context(AppError)
                .attach_printable("failed to determine current tracking status")?
            {
                return Err(Report::from(AppError))
                    .attach_printable("time tracker is already running")
                    .attach(Suggestion("use the `stop` command to stop tracking first"));
            }
            tracker
                .start()
                .change_context(AppError)
                .attach_printable("failed to start tracking")?;
        }
        Command::Stop => {
            if tracker
                .is_tracking()
                .change_context(AppError)
                .attach_printable("failed to determine current tracking status")?
            {
                tracker
                    .stop()
                    .change_context(AppError)
                    .attach_printable("failed to stop tracking")?;
            }
        }
    }

    Ok(())
}
