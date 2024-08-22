use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::common::UtcToLocal;

use super::tracker::TimeRecord;

/// The interval of time to use when generating a report.
#[derive(Debug, Clone, Copy)]
pub enum ReportTimespan {
    /// Today, 00:00 to 23:59
    Today,
}

/// Tracking statistics
#[derive(Debug, Clone)]
pub struct TrackingReport {
    records: Vec<TimeRecord>,
}

impl TrackingReport {
    pub fn new(records: Vec<TimeRecord>) -> Self {
        Self { records }
    }

    /// Returns the total duration tracked.
    ///
    /// Calculation will use the complete `timespan` starting at the `from` time.
    pub fn duration(&self, from: DateTime<Utc>, timespan: ReportTimespan) -> Duration {
        match timespan {
            ReportTimespan::Today => {
                let now = from.to_local();

                let midnight = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
                let end_of_day = now.date_naive().and_hms_opt(23, 59, 59).unwrap();

                let total_seconds = self
                    .records
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

        let report = TrackingReport::new(records);
        let duration = report.duration(now, ReportTimespan::Today);

        assert_eq!(duration, Duration::from_secs(15));
    }
}
