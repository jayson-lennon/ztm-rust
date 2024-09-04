use std::path::PathBuf;

use chrono::Utc;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use error_stack::{Report, Result, ResultExt};
use track::{
    error::{AppError, Suggestion},
    feature::{
        backend::{
            FlatFileReporter, FlatFileTracker, ReportTimespan, Reporter, Tracker, TWENTY_FOUR_HOURS,
        },
        report_fmt::{DurationFormat, HourMinSecFormatter},
    },
    init::{self},
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

    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start tracking time
    Start,

    /// Stop tracking time
    Stop,

    /// Report tracked time for the last 24 hours
    Report,
}

fn main() -> Result<(), AppError> {
    init::error_reporting();

    let args = Cli::parse();

    init::tracing(&args.verbose);

    let db_dir = flatfile_db_dir(&args)?;
    tracing::debug!(db_dir = %db_dir.display(), "using database path");

    let lockfile = lockfile_path(&args)?;
    tracing::debug!(lockfile = %lockfile.display(), "using lockfile path");

    let mut tracker = FlatFileTracker::new(db_dir, lockfile);

    match args.command {
        Command::Start => {
            if tracker
                .running()
                .change_context(AppError)
                .attach_printable("failed to determine current tracking status")?
                .is_some()
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
                .running()
                .change_context(AppError)
                .attach_printable("failed to determine current tracking status")?
                .is_some()
            {
                tracker
                    .stop()
                    .change_context(AppError)
                    .attach_printable("failed to stop tracking")?;
            }
        }
        Command::Report => {
            let reporter = FlatFileReporter::new(&tracker);

            let last_24_hours = Utc::now() - TWENTY_FOUR_HOURS;
            let duration = {
                let duration = reporter
                    .total_duration(ReportTimespan::Since(last_24_hours))
                    .change_context(AppError)
                    .attach_printable(
                        "failed to calculate total duration over the past 24 hours",
                    )?;
                HourMinSecFormatter.format(duration)
            };
            println!("{duration}");
        }
    }

    Ok(())
}

fn flatfile_db_dir(args: &Cli) -> Result<PathBuf, AppError> {
    match &args.db_dir {
        Some(db_dir) => Ok(db_dir.to_owned()),
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
            Ok(db_dir)
        }
    }
}

fn lockfile_path(args: &Cli) -> Result<PathBuf, AppError> {
    match &args.lockfile {
        Some(lockfile) => Ok(lockfile.to_owned()),
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
            Ok(lockfile)
        }
    }
}
