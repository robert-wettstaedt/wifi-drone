use constants;
use command::Command;
use keyboard::Keyboard;

use std::error::Error;
use std::{thread, time};
use std::net::UdpSocket;

pub struct Gamepad {
    socket: UdpSocket,
}

impl Gamepad {
    pub fn new() -> Gamepad {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(e) => panic!("Error connecting to gamepad socket: {}", e.description()),
        };

        return Gamepad { socket: socket };
    }

    pub fn start(mut self) {
        let mut keyboard = Keyboard::new();
        keyboard.start();

        self.start_input_listener(keyboard);
    }

    fn start_input_listener(&mut self, mut keyboard: Keyboard) {
        let mut cmd = Command::new();

        loop {
            keyboard.get_pressed_keys(&mut cmd);
            println!("{:?}", cmd);

            self.write(&mut cmd);

            let interval = time::Duration::from_millis(50);
            thread::sleep(interval);
        }
    }

    fn write(&mut self, cmd: &mut Command) {
        cmd.update_array();
        match self.socket.send_to(&(cmd.as_array), format!("{}:{}", constants::DRONE_HOST, constants::DRONE_UDP_PORT)) {
            Ok(_) => debug!("Sent gamepad data"),
            Err(e) => panic!("Error writing gamepad data: {}", e.description()),
        }
    }
}