use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use error_stack::{Result, ResultExt};

use crate::{
    error::Suggestion,
    feature::{report_fmt::DurationFormat, tracker::FlatFileTracker},
};

use super::{
    report_fmt::HMSFormatter,
    tracker::{ReportTimespan, Reporter, StartupStatus, Tracker},
};

#[derive(Debug, thiserror::Error)]
#[error("a CLI error occurred")]
pub struct CliError;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    /// Stop tracking time
    Stop,
    /// Generate report
    Report,
}

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    /// Path to database file
    #[arg(short = 'd', long)]
    pub db_dir: Option<PathBuf>,

    /// Path to lockfile
    #[arg(short = 'l', long)]
    pub lockfile: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

pub fn run() -> Result<(), CliError> {
    let args = Cli::parse();

    let db_dir = flatfile_db_dir(&args)?;
    let lockfile = lockfile_path(&args)?;

    let mut tracker = FlatFileTracker::new(db_dir, lockfile);

    match args.command {
        Command::Start => match tracker.start() {
            Ok(StartupStatus::Started) => (),
            Ok(StartupStatus::Running) => println!("tracker already running"),
            Err(e) => return Err(e).change_context(CliError),
        },
        Command::Stop => tracker
            .stop()
            .change_context(CliError)
            .attach_printable("failed to stop tracking")?,
        Command::Report => {
            // 1. how far back in time to look
            let twenty_four_hours = {
                const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24;
                Duration::from_secs(TWENTY_FOUR_HOURS)
            };

            // 2. reporter::total_duration
            let duration = tracker
                .total_duration(ReportTimespan::Last(twenty_four_hours))
                .change_context(CliError)
                .attach_printable("failed to calculate total tracked duration")?;

            // 3. format it

            let formatter = HMSFormatter::default();

            // 4. print
            println!("{}", formatter.format(duration));
        }
    }

    Ok(())
}

fn flatfile_db_dir(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.db_dir {
        Some(db_dir) => Ok(db_dir.clone()),
        None => {
            let mut db_path = dirs::data_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover data directory")
                .attach(Suggestion("use the -d flag to specify a database path"))?;

            db_path.push("track");

            std::fs::create_dir_all(&db_path)
                .change_context(CliError)
                .attach_printable("failed to created 'track' db directory")?;

            db_path.push("records.json");
            Ok(db_path)
        }
    }
}

fn lockfile_path(args: &Cli) -> Result<PathBuf, CliError> {
    match &args.lockfile {
        Some(lockfile) => Ok(lockfile.clone()),
        None => {
            let mut lockfile = dirs::cache_dir()
                .ok_or(CliError)
                .attach_printable("failed to discover cache directory")
                .attach(Suggestion("use the -l flag to specify a lockfile path"))?;

            lockfile.push("track");

            std::fs::create_dir_all(&lockfile)
                .change_context(CliError)
                .attach_printable("failed to created 'track' cache directory")?;

            lockfile.push("track.lock");
            Ok(lockfile)
        }
    }
}
