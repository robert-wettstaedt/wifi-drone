#[macro_use] extern crate log;
extern crate env_logger;

mod constants;
mod gamepad;
mod heartbeat;
mod video;

use std::error::Error;

use std::net::{TcpStream};
use std::io::{Write};

pub fn connect() {
    env_logger::init().unwrap();

    let mut handshake_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Error connecting to handshake socket: {}", e.description()),
    };
    let files = ["handshake", "video_1_1", "video_1_2"];
    for file in files.into_iter() {
        let path = format!("res/{}.dat", file).to_string();
        match handshake_stream.write(constants::read_file(&path).as_slice()) {
            Ok(_) => debug!("Sent {}", file),
            Err(e) => panic!("Error writing {}: {}", file, e.description()),
        }
    }

    gamepad::Gamepad::new().start();
    heartbeat::Heartbeat::new().start();
    video::Video::new().start();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_valid() {
        connect();
    }
}
