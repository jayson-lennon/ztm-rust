use assert_fs::{
    fixture::{ChildPath, FixtureError},
    prelude::*,
    TempDir,
};

use crate::feature::{
    gui::Message,
    tracker::{FlatFileTracker, TimeTracker},
};

use super::GuiApp;

fn tracking_paths() -> Result<(TempDir, ChildPath, ChildPath, impl TimeTracker), FixtureError> {
    let temp = TempDir::new()?;
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");
    let tracker = FlatFileTracker::new(db.path(), lockfile.path()).unwrap();
    Ok((temp, db, lockfile, tracker))
}

#[test]
fn starts_tracking() {
    let (_tree, _db, _lockfile, tracker) = tracking_paths().unwrap();

    let mut app = GuiApp::new(Box::new(tracker)).unwrap();

    app.update(Message::StartTracking);

    assert!(app.start_time.is_some());
}

#[test]
fn tracking_error_displayed_when_starting_multiple_times() {
    // Notes: The "start tracking" button should disallow this from happening. But just in case the
    // message is sent from elsewhere, we should display an error noting that the tracker is
    // already running.
    let (_tree, _db, _lockfile, tracker) = tracking_paths().unwrap();

    let mut app = GuiApp::new(Box::new(tracker)).unwrap();

    app.update(Message::StartTracking);
    app.update(Message::StartTracking);

    assert!(app.tracker_error.is_some());
}

#[test]
fn stops_tracking() {
    let (_tree, _db, _lockfile, tracker) = tracking_paths().unwrap();

    let mut app = GuiApp::new(Box::new(tracker)).unwrap();

    app.update(Message::StartTracking);
    app.update(Message::StopTracking);

    assert!(app.start_time.is_none());
}
