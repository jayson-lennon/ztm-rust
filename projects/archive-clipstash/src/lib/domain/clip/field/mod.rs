//! Fields for the [`Clip`](crate::Clip) data type.

mod clip_id;
pub use clip_id::ClipId;

mod shortcode;
pub use shortcode::ShortCode;

mod content;
pub use content::Content;

mod title;
pub use title::Title;

mod posted;
pub use posted::Posted;

mod expires;
pub use expires::Expires;

mod password;
pub use password::Password;

mod hits;
pub use hits::Hits;
