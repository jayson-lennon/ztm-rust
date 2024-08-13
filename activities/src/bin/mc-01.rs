// Topic: Maintainable code via traits
//
// Summary:
//   Recently there was a power outage and all of the messages stored in the message queue
//   were lost. You have been tasked with adding functionality to save and load the queue. Review
//   the code and then implement the requirements as detailed below.
//
// Requirements:
// - Create a trait named `MessageQueueStorage` that allows the entire queue to be saved and loaded
//   - The trait should have 2 methods:
//     - `save(&self, queue: &MessageQueue) -> Result<(), MessageQueueStorageError>;`
//     - `load(&self) -> Result<MessageQueue, MessageQueueStorageError>;`
// - Create a struct named `FileStore` and then implement the `MessageQueueStorage` trait on it
//   - The implementation should save the entire queue to a single file and also load it from a
//     single file
//   - Implement a `new` method which allows specifying the file path
// - Use the provided `FileStoreError` type for errors that occur in your implementation and then
//   convert it to `MessageQueueStorageError` in the trait method
//   - This can be done automatically by using the question mark operator
// - Run `cargo test --bin mc-01` to check your work
//
// Tips:
// - You'll need to serialize and deserialize the message queue
//   - Serialize: read each entry in the queue and then save them to a file
//   - Deserialize: read each entry from the file and then create a new queue
//
// - The storage format is left unspecified. Here are a few options:
//   - Comma-separated values (CSV) format:
//     - Format each message by `id,content`
//   - JSON format:
//     - add `#[derive(Serialize, Deserialize)]` to the message queue
//     - use the `serde_json` crate to perform the serialize and deserialize operation

use color_eyre::eyre::eyre;
use std::collections::VecDeque;

/// A message in the queue.
///
/// ***********************
/// Do not edit the message
/// ***********************
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message {
    pub id: u32,
    pub content: String,
}

impl Message {
    /// Create a new message.
    pub fn new<S: Into<String>>(id: u32, content: S) -> Self {
        Self {
            id,
            content: content.into(),
        }
    }
}

/// An error that may occur while saving and loading the queue using a storage backend.
///
/// ***************************************************************************
/// Do not edit this error type. It is part of the `MessageQueueStorage` trait.
/// ***************************************************************************
#[derive(Debug, thiserror::Error)]
#[error("message queue storage error")]
struct MessageQueueStorageError {
    // this allows putting any errors as a source
    source: color_eyre::Report,
}

/// Errors that may occur while working with the `FileStore`.
///
/// ***************************************************
/// Change this enum as needed for your implementation.
/// ***************************************************
#[derive(Debug, thiserror::Error)]
enum FileStoreError {
    #[error("IO error")]
    IO(#[from] std::io::Error),
    // add more variants if needed
}

/// Allows conversion of error type using question mark operator.
///
/// *****************************
/// You can convert a `FileStoreError` to a `MessageQueueStorageError` using `map_err`:
///
///    fn foo() -> Result<(), MessageQueueStorageError> {
///        do_fallible_thing().map_err(MessageQueueStorageError::from)
///    }
///
/// You can also use the question mark operator:
///
///    fn foo() -> Result<(), MessageQueueStorageError> {
///        let result = do_fallible_thing()?;
///        Ok(result)
///    }
///
/// or
///
///    fn foo() -> Result<(), MessageQueueStorageError> {
///        Ok(do_fallible_thing()?);
///    }
/// *****************************
impl From<FileStoreError> for MessageQueueStorageError {
    fn from(value: FileStoreError) -> Self {
        Self {
            source: eyre!(value),
        }
    }
}

/// A message queue.
///
/// *****************************
/// Do not edit the message queue
/// *****************************
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageQueue {
    messages: VecDeque<Message>,
    next_id: u32,
}

impl MessageQueue {
    /// Add a new message to the queue.
    pub fn enqueue<M: Into<String>>(&mut self, message: M) {
        let message = Message {
            id: self.next_id,
            content: message.into(),
        };
        self.messages.push_back(message);
        self.next_id += 1;
    }

    /// Remove and return the first message in the queue.
    pub fn dequeue(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    /// Iterate over all messages in the queue.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, Message> {
        self.messages.iter()
    }
}

/********************************************
* Add your code here:
* - `MessageQueueStorage` trait
* - `FileStore` struct
* - implementation blocks
********************************************/

/// *****************************************************************
/// use `cargo test --bin mc-01` to check your work.
/// *****************************************************************
/// use `cargo run --bin mc-01` to experiment using the main function
/// *****************************************************************
fn main() -> color_eyre::Result<()> {
    // show pretty error output
    color_eyre::install().unwrap();

    let mut queue = MessageQueue::default();
    queue.enqueue("first message");
    queue.enqueue("second message");
    Ok(())

    // save/load here
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_NAME: &str = ".mc-01-test";

    fn cleanup() {
        let _ = std::fs::remove_file(TEST_FILE_NAME);
    }

    #[test]
    fn queue_saves_and_loads_correctly() {
        color_eyre::install().unwrap();
        let test = || -> Result<(), color_eyre::Report> {
            let mut queue = MessageQueue::default();
            queue.enqueue("a");
            queue.enqueue("b");
            queue.dequeue();
            queue.enqueue("c");

            let storage = FileStore::new(".mc-01-test");
            storage.save(&queue)?;

            let loaded_queue = storage.load()?;
            cleanup();

            assert_eq!(loaded_queue, queue);
            Ok(())
        };

        let results = test();
        cleanup();
        results.expect("test failed");
    }
}
