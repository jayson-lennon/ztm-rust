pub fn trim(msg: &str) -> &str {
    msg.trim()
}

pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
    if let Some(letter) = msg.get(0..1) {
        format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
    } else {
        msg.into()
    }
}

pub fn exciting(msg: &str) -> String {
    format!("{}!", msg)
}
