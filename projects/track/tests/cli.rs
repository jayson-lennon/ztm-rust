use assert_cmd::Command;
use assert_fs::{prelude::*, TempDir};
use testresult::TestResult;

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
    let temp = TempDir::new()?;
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
        .arg("start")
        .assert()
        .success();

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.path())
        .arg("--lockfile")
        .arg(lockfile.path())
        .arg("stop")
        .assert()
        .success();

    // make sure the database file isn't empty
    let meta = std::fs::metadata(db.path())?;
    assert!(meta.len() > 0);

    Ok(())
}
