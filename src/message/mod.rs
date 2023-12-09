use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub size: usize,
    pub kind: MessageKind,
    pub command: Command,
    pub contents: String,
}

impl Message {
    pub fn _empty_message() -> Message {
        return Message {
            size: 0,
            kind: MessageKind::Empty,
            command: Command::NA,
            contents: String::new(),
        }
    }
    pub fn get_contents(&self) -> String {
        return self.contents.clone();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageKind {
    Empty,
    File(String), // File name. e.g. file_name.extension
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    NA,
    Store(String), // Path name. e.g. /path/to/store/data/
    Read,
}
