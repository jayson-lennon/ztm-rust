use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(|| {});
}

fn change_data(display_data: SharedSignData, new_data: &str) {}

fn main() {
    let display_data = Arc::new(Mutex::new("initial".to_owned()));
    spawn_display_thread(Arc::clone(&display_data));

    thread::sleep(Duration::from_millis(100));
    change_data(Arc::clone(&display_data), "message 1");
    thread::sleep(Duration::from_millis(500));
    change_data(Arc::clone(&display_data), "another message");
    thread::sleep(Duration::from_millis(500));
    change_data(Arc::clone(&display_data), "goodbye");
    thread::sleep(Duration::from_millis(500));
}
