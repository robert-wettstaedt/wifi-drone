use constants;
use controls::command::Command;
use window_manager::WindowManager;

use std::error::Error;
use std::{thread, time};
use std::net::UdpSocket;

pub struct Gamepad <'a>  {
    socket: UdpSocket,
    window_manager: &'a WindowManager
}

impl <'a> Gamepad <'a> {
    pub fn new(window_manager: &WindowManager) -> Gamepad {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(e) => panic!("Error connecting to gamepad socket: {}", e.description()),
        };

        return Gamepad { socket, window_manager: &window_manager };
    }

    pub fn start(mut self) {
        self.start_input_listener();
    }

    fn start_input_listener(&mut self) {
        let mut cmd = Command::new();

        loop {
            self.window_manager.get_pressed_keys(&mut cmd);
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