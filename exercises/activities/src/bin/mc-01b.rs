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

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::num::ParseIntError;
use std::path::PathBuf;

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
#[derive(Debug)]
struct MessageQueueStorageError {
    // this allows putting any errors as a source
    source: FileStoreError,
}

impl fmt::Display for MessageQueueStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message queue storage error: {}", self.source)
    }
}

/// Errors that may occur while working with the `FileStore`.
///
/// ***************************************************
/// Change this enum as needed for your implementation.
/// ***************************************************
#[derive(Debug)]
enum FileStoreError {
    Io(std::io::Error),
    WrongFormat,
    ParseId(ParseIntError),
}

impl fmt::Display for FileStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::WrongFormat => write!(f, "invalid message line format"),
            Self::ParseId(error) => write!(f, "invalid message ID: {error}"),
        }
    }
}

impl Error for FileStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::ParseId(error) => Some(error),
            Self::WrongFormat => None,
        }
    }
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
// impl From<FileStoreError> for MessageQueueStorageError {
//     fn from(value: FileStoreError) -> Self {
//         Self {
//             source: eyre!(value),
//         }
//     }
// }

impl From<std::io::Error> for FileStoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ParseIntError> for FileStoreError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseId(error)
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
trait MessageQueueStorage {
    fn save(&self, queue: &MessageQueue) -> Result<(), MessageQueueStorageError>;
    fn load(&self) -> Result<MessageQueue, MessageQueueStorageError>;
}

impl From<FileStoreError> for MessageQueueStorageError {
    fn from(source: FileStoreError) -> Self {
        Self { source }
    }
}

impl Error for MessageQueueStorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

struct FileStore {
    path: PathBuf,
}

impl FileStore {
    fn load0(&self) -> Result<MessageQueue, FileStoreError> {
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
            let parts: Vec<_> = line.splitn(2, ',').collect();
            if parts.len() != 2 {
                return Err(FileStoreError::WrongFormat)?;
            }

            let id = parts[0].parse::<u32>().map_err(FileStoreError::ParseId)?;
            let content = parts[1].to_string();
            messages.push_back(Message::new(id, content));
        }

        let next_id = messages.iter().map(|msg| msg.id).max().unwrap_or_default() + 1;
        Ok(MessageQueue { messages, next_id })
    }
}

impl FileStore {
    fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { path: path.into() }
    }

    fn save0(&self, queue: &MessageQueue) -> Result<(), FileStoreError> {
        let tmp_path = {
            let mut path = self.path.clone();
            path.set_extension(".tmp");
            path
        };
        let mut writer = {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)?;
            BufWriter::new(file)
        };

        for msg in queue.iter() {
            writeln!(writer, "{},{}", msg.id, msg.content)?;
        }
        fs::rename(tmp_path, &self.path)?;

        Ok(())
    }
}

impl MessageQueueStorage for FileStore {
    fn save(&self, queue: &MessageQueue) -> Result<(), MessageQueueStorageError> {
        Ok(self.save0(queue)?)
    }

    fn load(&self) -> Result<MessageQueue, MessageQueueStorageError> {
        Ok(self.load0()?)
    }
}

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
    use std::error::Error;

    const TEST_FILE_NAME: &str = ".mc-01-test";

    fn cleanup() {
        let _ = fs::remove_file(TEST_FILE_NAME);
    }

    #[test]
    fn queue_saves_and_loads_correctly() -> Result<(), Box<dyn Error>> {
        cleanup();

        let result = (|| -> Result<(), Box<dyn Error>> {
            let mut queue = MessageQueue::default();
            queue.enqueue("a");
            queue.enqueue("b");
            queue.dequeue();
            queue.enqueue("c, including a comma");

            let storage = FileStore::new(TEST_FILE_NAME);
            storage.save(&queue)?;

            let loaded_queue = storage.load()?;
            assert_eq!(loaded_queue, queue);

            Ok(())
        })();

        cleanup();
        result
    }
}
