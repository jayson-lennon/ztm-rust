//! Background database maintenance task.

use crate::data::DatabasePool;
use crate::service;
use std::time::Duration;
use tokio::runtime::Handle;

/// Async background task that performs rountine database tasks.
///
/// This task deletes expired clips periodically.
pub struct Maintenance;

impl Maintenance {
    /// Spawn the database maintenance task.
    pub fn spawn(pool: DatabasePool, handle: Handle) -> Self {
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = service::action::delete_expired(&pool).await {
                    eprintln!("failed to delete expired clips: {}", e);
                }
            }
        });
        Self
    }
}