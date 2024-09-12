mod ex1 {

    struct Uppercase(String);

    impl From<String> for Uppercase {
        fn from(data: String) -> Self {
            Uppercase(data.to_uppercase())
        }
    }

    impl From<&str> for Uppercase {
        fn from(data: &str) -> Self {
            Uppercase(data.to_uppercase())
        }
    }

    pub fn run() {
        let upper = Uppercase::from("lowercase");
        let upper: Uppercase = "lowercase".into();
    }
}

mod ex2 {

    #[derive(Clone, Copy)]
    enum KeyPress {
        Down,
        Up,
    }

    #[derive(Clone, Copy)]
    struct KeyEvent {
        keycode: u16,
        state: KeyPress,
    }

    enum InputEvent {
        Key(u16, KeyPress),
        Mouse,
    }

    impl From<KeyEvent> for InputEvent {
        fn from(ev: KeyEvent) -> Self {
            InputEvent::Key(ev.keycode, ev.state)
        }
    }

    pub fn run() {
        let key_ev = KeyEvent {
            keycode: 5,
            state: KeyPress::Down,
        };

        let input_ev = InputEvent::from(key_ev);
        let input_ev: InputEvent = key_ev.into();
    }
}

mod ex3 {

    use thiserror::Error;

    #[derive(Debug, Error)]
    enum NetworkError {
        #[error("connection timed out")]
        Timeout,
    }

    #[derive(Debug, Error)]
    enum DatabaseError {
        #[error("error querying database")]
        QueryFailure,
    }

    #[derive(Debug, Error)]
    enum ApiError {
        #[error("network error: {0}")]
        Network(#[from] NetworkError),
        #[error("database error: {0}")]
        Database(#[from] DatabaseError),
    }

    fn do_stuff() -> Result<(), ApiError> {
        Err(NetworkError::Timeout)?
    }
}

fn main() {
    ex1::run();
    ex2::run();
}
