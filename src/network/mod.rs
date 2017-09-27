pub mod heartbeat;
pub mod video;
pub mod gamepad;

use super::constants;
use super::glutin::{ElementState, VirtualKeyCode};

use self::heartbeat::Heartbeat;
use self::video::Video;
use self::gamepad::{Gamepad, CommandListener};

use std::error::Error;
use std::net::TcpStream;
use std::io::Write;
use std::sync::mpsc::Receiver;

pub fn start(keypress_rx: Receiver<Vec<VirtualKeyCode>>, command_listener: CommandListener) {
    let mut handshake_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Error connecting to handshake socket: {}", e.description()),
    };

    let functions = [constants::get_handshake_1, constants::get_handshake_2, constants::get_handshake_3];
    for index in 0..functions.len() {
        match handshake_stream.write(functions[index]().as_slice()) {
            Ok(_) => debug!("Sent {}", index),
            Err(e) => panic!("Error writing {}: {}", index, e.description()),
        }
    }

    Heartbeat::new().start();
    Video::new().start();
    Gamepad::new(keypress_rx, command_listener).start();
}