use assert_cmd::Command;
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

#[test]
fn error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();

    Ok(())
}

#[test]
fn error_if_unknown_command_specified() -> TestResult {
    Command::cargo_bin("track")?
        .arg("whoops")
        .assert()
        .failure();

    Ok(())
}

#[test]
fn tracks_time_when_starting_then_stopping_tracker() -> TestResult {
    let (_tree, db, lockfile) = tracking_paths().unwrap();

    start_tracking(&db, &lockfile)?;
    stop_tracking(&db, &lockfile)?;

    // make sure the database file isn't empty
    let meta = std::fs::metadata(db.path())?;
    assert!(meta.len() > 0);

    Ok(())
}

#[test]
fn reports_time_tracked() -> TestResult {
    let (_tree, db, lockfile) = tracking_paths().unwrap();

    start_tracking(&db, &lockfile)?;
    stop_tracking(&db, &lockfile)?;

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
        .arg("report")
        .assert()
        .stdout("00:00:00\n")
        .success();

    Ok(())
}
