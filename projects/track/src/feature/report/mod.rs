//! Statistics reporting

pub mod duration_format;

use super::tracker::TimeRecord;
use crate::common::UtcToLocal;
use chrono::{DateTime, Utc};
use std::time::Duration;

/// The interval of time to use when generating a report.
#[derive(Debug, Clone, Copy)]
pub enum ReportTimespan {
    /// Today, 00:00 to 23:59
    Today,
}

/// Tracking statistics
#[derive(Debug, Clone)]
pub struct TrackingReport;

impl TrackingReport {
    /// Returns the total duration tracked.
    ///
    /// Calculation will use the complete `timespan` starting at the `from` time.
    pub fn duration(
        &self,
        from: DateTime<Utc>,
        timespan: ReportTimespan,
        records: &[TimeRecord],
    ) -> Duration {
        match timespan {
            ReportTimespan::Today => {
                let now = from.to_local();

                let midnight = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
                let end_of_day = now.date_naive().and_hms_opt(23, 59, 59).unwrap();

                let total_seconds = records
                    .iter()
                    .filter_map(|rec| {
                        let start_local = rec.start.to_local().naive_local();
                        let end_local = rec.end.to_local().naive_local();

                        if start_local >= midnight && end_local <= end_of_day {
                            Some(
                                end_local.and_utc().timestamp() - start_local.and_utc().timestamp(),
                            )
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();
                Duration::from_secs(total_seconds as u64)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::tracker::{tlib::TimeRecordTestExt, TimeRecord};
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn report_calculates_correct_duration_with_multiple_records() {
        let now = Utc::now();
        let ts = now.timestamp();

        let records = [
            // (start, end)
            (ts - 30, ts - 20), // 30 sec ago to 20 sec ago (10 sec duration)
            (ts - 20, ts - 15), // 20 sec ago to 15 sec ago (5 sec duration)
        ]
        .into_iter()
        .map(TimeRecord::from_seconds)
        .collect::<Vec<_>>();

        let report = TrackingReport;
        let duration = report.duration(now, ReportTimespan::Today, &records);

        assert_eq!(duration, Duration::from_secs(15));
    }
}
