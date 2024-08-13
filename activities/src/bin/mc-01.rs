// Topic: Maintainable code via traits
//
// Summary:
//   Recently there was a power outage and all of the messages stored in the below message queue
//   were lost. You have been tasked with adding functionality to save and load the queue.
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

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::path::PathBuf;

use color_eyre::eyre::eyre;

/// A message in the queue.
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
#[derive(Debug, thiserror::Error)]
#[error("message queue storage error")]
struct MessageQueueStorageError {
    source: color_eyre::Report,
}

/// Errors that may occur while working with the `FileStore`.
#[derive(Debug, thiserror::Error)]
enum FileStoreError {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    // add more variants as needed
    #[error("invalid line format")]
    WrongFormat,

    #[error("ID parse error")]
    ParseId(#[from] ParseIntError),
}

/// Allows conversion of error type using question mark operator.
impl From<FileStoreError> for MessageQueueStorageError {
    fn from(value: FileStoreError) -> Self {
        Self {
            source: eyre!(value),
        }
    }
}

trait MessageQueueStorage {
    fn save(&self, queue: &MessageQueue) -> Result<(), MessageQueueStorageError>;
    fn load(&self) -> Result<MessageQueue, MessageQueueStorageError>;
}

/// Store a message queue to a file
struct FileStore {
    path: PathBuf,
}

impl MessageQueueStorage for FileStore {
    fn save(&self, queue: &MessageQueue) -> Result<(), MessageQueueStorageError> {
        Ok(self.save_impl(queue)?)
    }

    fn load(&self) -> Result<MessageQueue, MessageQueueStorageError> {
        Ok(self.load_impl()?)
    }
}

impl FileStore {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    fn save_impl(&self, queue: &MessageQueue) -> Result<(), FileStoreError> {
        use std::fs::OpenOptions;
        use std::io::BufWriter;
        use std::io::Write;

        let scratch_path = {
            let mut path = self.path.clone();
            path.set_extension(".tmp");
            path
        };

        let mut writer = {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&scratch_path)?;

            BufWriter::new(file)
        };

        for msg in queue.iter() {
            writeln!(writer, "{},{}", msg.id, msg.content)?;
        }

        std::fs::rename(scratch_path, &self.path)?;
        Ok(())
    }

    fn load_impl(&self) -> Result<MessageQueue, FileStoreError> {
        use std::fs::OpenOptions;
        use std::io::BufRead;
        use std::io::BufReader;

        let reader = {
            let file = OpenOptions::new()
                .read(true)
                .open(&self.path)
                .map_err(FileStoreError::from)?;
            BufReader::new(file)
        };

        let mut messages = VecDeque::default();

        for line in reader.lines() {
            let line = line.map_err(FileStoreError::from)?;
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            if parts.len() != 2 {
                return Err(FileStoreError::WrongFormat)?;
            }
            let id = parts[0].parse::<u32>().map_err(FileStoreError::ParseId)?;
            let content = parts[1].to_string();
            messages.push_back(Message { id, content });
        }

        let next_id = messages.iter().map(|msg| msg.id).max().unwrap_or_default() + 1;

        Ok(MessageQueue { messages, next_id })
    }
}

/// A message queue.
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

fn main() -> color_eyre::Result<()> {
    // show pretty error output
    color_eyre::install().unwrap();

    // you can use this sample queue to iterate on your work
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
