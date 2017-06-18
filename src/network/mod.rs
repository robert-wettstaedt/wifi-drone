pub mod heartbeat;
pub mod video;
pub mod gamepad;

use super::constants;
use self::heartbeat::Heartbeat;
use self::video::Video;
use self::gamepad::Gamepad;

use std::error::Error;
use std::net::TcpStream;
use std::io::Write;

pub fn start() {
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