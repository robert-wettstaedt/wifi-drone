#[macro_use] extern crate log;
extern crate env_logger;

mod command;
mod constants;
mod gamepad;
mod heartbeat;
mod keyboard;
mod video;

use gamepad::Gamepad;
use heartbeat::Heartbeat;
use video::Video;

use std::error::Error;
use std::net::TcpStream;
use std::io::Write;

pub fn connect() {
    env_logger::init().unwrap();

    let mut handshake_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Error connecting to handshake socket: {}", e.description()),
    };

    let functions = [constants::get_handshake, constants::get_video_1_1, constants::get_video_1_2];
    for index in 0..functions.len() {
        match handshake_stream.write(functions[index]().as_slice()) {
            Ok(_) => debug!("Sent {}", index),
            Err(e) => panic!("Error writing {}: {}", index, e.description()),
        }
    }

    Heartbeat::new().start();
    Video::new().start();
    Gamepad::new().start();
}

// #[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_connect_valid() {
//        connect();
//    }
//}
