use super::{load_records, FlatFileTracker};
use crate::feature::backend::{
    reporter::{ReportTimespan, Reporter, ReporterError},
    TimeRecord,
};
use error_stack::{Result, ResultExt};
use std::time::Duration;
use tracing::instrument;

/// Generate statistics for flat file backend.
#[derive(Clone, Debug)]
pub struct FlatFileReporter<'a> {
    tracker: &'a FlatFileTracker,
}

impl<'a> FlatFileReporter<'a> {
    pub fn new(tracker: &'a FlatFileTracker) -> Self {
        Self { tracker }
    }
}

impl<'a> Reporter for FlatFileReporter<'a> {
    #[instrument]
    fn total_duration(&self, timespan: ReportTimespan) -> Result<Duration, ReporterError> {
        match timespan {
            ReportTimespan::Since(start) => {
                let total_ms = self
                    .records()
                    .change_context(ReporterError)
                    .attach_printable("failed to query records from flat file tracker")?
                    .filter_map(|rec| {
                        if rec.start >= start {
                            let ms = rec.end.timestamp_millis() - rec.start.timestamp_millis();
                            Some(ms)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();
                Ok(Duration::from_millis(total_ms as u64))
            }
        }
    }

    #[instrument]
    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, ReporterError> {
        load_records(&self.tracker.records)
            .change_context(ReporterError)
            .attach_printable("failed to load flat file tracker records")
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::feature::backend::{
        flatfile::tracker::tests::{new_flat_file_tracker, tracking_paths},
        tracker::Tracker,
    };

    use super::*;

    #[test]
    fn calculates_correct_duration_when_there_are_no_records() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();
        let tracker = new_flat_file_tracker(db, &lockfile);

        let reporter = FlatFileReporter::new(&tracker);

        let duration = reporter
            .total_duration(ReportTimespan::Since(Utc::now() - Duration::from_secs(1)))
            .unwrap();

        assert_eq!(duration, Duration::from_millis(0));
    }

    #[test]
    fn calculates_correct_duration_when_there_are_two_records() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();
        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        tracker.start().unwrap();
        std::thread::sleep(Duration::from_millis(10));
        tracker.stop().unwrap();

        let reporter = FlatFileReporter::new(&tracker);

        let duration = reporter
            .total_duration(ReportTimespan::Since(Utc::now() - Duration::from_secs(1)))
            .unwrap();

        assert!(duration >= Duration::from_millis(20));
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        let (_tree, db, lockfile) = tracking_paths().unwrap();

        let mut tracker = new_flat_file_tracker(db, &lockfile);
        tracker.start().unwrap();
        tracker.stop().unwrap();

        let reporter = FlatFileReporter::new(&tracker);

        assert!(reporter.records().unwrap().next().is_some());
    }
}
