use assert_cmd::Command;
<<<<<<< Updated upstream
use assert_fs::{
    fixture::{ChildPath, FixtureError},
    prelude::*,
    TempDir,
};
use testresult::{TestError, TestResult};

fn stop_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
        .arg("stop")
        .assert()
        .success();
    Ok(())
}

fn start_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
        .arg("start")
        .assert()
        .success();
    Ok(())
}

fn tracking_paths() -> Result<(TempDir, ChildPath, ChildPath), FixtureError> {
    let temp = TempDir::new()?;
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");
    Ok((temp, db, lockfile))
}
=======
use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};
use testresult::TestResult;
>>>>>>> Stashed changes

fn tracking_paths() -> (TempDir, ChildPath, ChildPath) {
    let temp = TempDir::new().unwrap();
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");
    (temp, db, lockfile)
}

fn start_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), testresult::TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("start")
        .assert()
        .success();
    Ok(())
}

fn stop_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), testresult::TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("stop")
        .assert()
        .success();
    Ok(())
}

#[test]
fn error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();

    Ok(())
}

#[test]
<<<<<<< Updated upstream
fn error_if_unknown_command_specified() -> TestResult {
    Command::cargo_bin("track")?
        .arg("whoops")
        .assert()
        .failure();

=======
fn start_command_starts_tracking_time() -> TestResult {
    let (_tempdir, db, lockfile) = tracking_paths();
    // track --db-dir PATH --lockfile PATH start
    start_tracking(&db, &lockfile)?;

    assert!(lockfile.to_path_buf().exists());

>>>>>>> Stashed changes
    Ok(())
}

#[test]
<<<<<<< Updated upstream
fn tracks_time_when_starting_then_stopping_tracker() -> TestResult {
    let (_tree, db, lockfile) = tracking_paths().unwrap();

    start_tracking(&db, &lockfile)?;
    stop_tracking(&db, &lockfile)?;

    // make sure the database file isn't empty
    let meta = std::fs::metadata(db.path())?;
    assert!(meta.len() > 0);
=======
fn stop_command_stops_tracking_time() -> TestResult {
    let (_tempdir, db, lockfile) = tracking_paths();

    start_tracking(&db, &lockfile)?;

    // track --db-dir PATH --lockfile PATH stop
    stop_tracking(&db, &lockfile)?;

    assert!(!lockfile.to_path_buf().exists());
>>>>>>> Stashed changes

    Ok(())
}

#[test]
<<<<<<< Updated upstream
fn reports_time_tracked() -> TestResult {
    let (_tree, db, lockfile) = tracking_paths().unwrap();
=======
fn report_command_generates_report() -> TestResult {
    let (_tempdir, db, lockfile) = tracking_paths();
>>>>>>> Stashed changes

    start_tracking(&db, &lockfile)?;
    stop_tracking(&db, &lockfile)?;

    Command::cargo_bin("track")?
        .arg("--db-dir")
<<<<<<< Updated upstream
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
=======
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
>>>>>>> Stashed changes
        .arg("report")
        .assert()
        .stdout("00:00:00\n")
        .success();

    Ok(())
}
