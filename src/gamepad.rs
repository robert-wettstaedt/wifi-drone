use constants;

use std::error::Error;

use std::net::{UdpSocket};

pub struct Gamepad {
    socket: UdpSocket,
    data: Vec<u8>,
}

impl Gamepad {
    pub fn new() -> Gamepad {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(e) => panic!("Error connecting to gamepad socket: {}", e.description()),
        };

        let data = constants::read_file("res/gamepad.dat");

        return Gamepad { socket: socket, data: data };
    }

    pub fn start(self) {
        match self.socket.send_to(self.data.as_slice(), format!("{}:{}", constants::DRONE_HOST, constants::DRONE_UDP_PORT)) {
            Ok(_) => debug!("Sent gamepad"),
            Err(e) => panic!("Error writing gamepad: {}", e.description()),
        }
    }
}


