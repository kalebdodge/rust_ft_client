pub mod message;
use crate::message::{Message, MessageKind, Command};

use std::{
    io::{ prelude::*, },
    net::{ SocketAddr, TcpStream },
    fs,
};
use serde::Deserialize;

fn main() {
    //let addr = SocketAddr::from(([192, 168, 1, 5], 8080));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    if let Ok(reader_stream) = TcpStream::connect(addr) {
        let writer_stream = reader_stream.try_clone().unwrap();
        println!("Connection established.");
        let file_content = read_file("./this_some_text");
        let writer_message = Message {
            size: file_content.len(),
            kind: MessageKind::File(String::from("/this_some_text")),
            command: Command::Store(String::from("./test")),
            contents: file_content,
        };

        write_to_stream(writer_stream, writer_message);
        let reader_message = read_from_stream(reader_stream);
        println!("{:#?}", reader_message);
    } else {
        println!("Could not establish connection.");
    }
}

fn read_from_stream(stream: TcpStream) -> Message {
    let mut buffer = serde_json::Deserializer::from_reader(stream);
    let message = Message::deserialize(&mut buffer).unwrap();
    return message;
}

fn write_to_stream(mut stream: TcpStream, message: Message) {
    let written = stream.write(
        serde_json::to_string(&message)
        .unwrap()
        .as_bytes()
    );
    println!("{} BYTES WRITTEN TO STREAM", written.unwrap());
}

fn read_file(path: &str) -> String {
    let mut contents = String::new();
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Could not find file: {}", e),
    };
    match file.read_to_string(&mut contents) {
        Ok(size) => println!("Read {} bytes from file", size),
        Err(e) => panic!("Could not write from file: {}", e),
    }
    return contents;
}
