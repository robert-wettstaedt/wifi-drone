use constants;
use keyboard::Keyboard;

use std::error::Error;
use std::{fmt, thread, time};
use std::net::UdpSocket;

pub struct Gamepad {
    socket: UdpSocket,
}

pub struct Move {
    pub pitch: i8,
    pub yaw: i8,
    pub roll: i8,
    pub throttle: i8,
    pub take_off: bool,
    pub land: bool,
    pub as_array: [u8; 8],
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
        let mut cmd = Move::new();

        loop {
            keyboard.get_pressed_keys(&mut cmd);
            debug!("{:?}", cmd);

            self.write(&mut cmd);

            let interval = time::Duration::from_millis(50);
            thread::sleep(interval);
        }
    }

    fn write(&mut self, cmd: &mut Move) {
        cmd.update_array();
        match self.socket.send_to(&(cmd.as_array), format!("{}:{}", constants::DRONE_HOST, constants::DRONE_UDP_PORT)) {
            Ok(_) => debug!("Sent gamepad data"),
            Err(e) => panic!("Error writing gamepad data: {}", e.description()),
        }
    }
}

impl Move {
    fn new() -> Move {
        Move { throttle: 0, yaw: 0, pitch: 0, roll: 0, land: false, take_off: false, as_array: [0; 8] }
    }

    fn update_array(&mut self) {
        self.as_array[0] = 0x66;

        if self.roll >= 0 {
            self.as_array[1] = (self.roll as u8) + 127;
        } else {
            self.as_array[1] = (self.roll + 127) as u8;
        }

        if self.pitch>= 0 {
            self.as_array[2] = (self.pitch as u8) + 127;
        } else {
            self.as_array[2] = (self.pitch + 127) as u8;
        }

        if self.throttle>= 0 {
            self.as_array[3] = (self.throttle as u8) + 127;
        } else {
            self.as_array[3] = (self.throttle + 127) as u8;
        }

        if self.yaw>= 0 {
            self.as_array[4] = (self.yaw as u8) + 127;
        } else {
            self.as_array[4] = (self.yaw + 127) as u8;
        }

        if self.take_off {
            self.as_array[5] = 0x01;
        } else {
            self.as_array[5] = 0x00;
        }

        self.as_array[6] = (self.as_array[1] ^ self.as_array[2] ^ self.as_array[3] ^ self.as_array[4] ^ self.as_array[5]) & 0xFF;

        self.as_array[7] = 0x99;
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "throttle: {}, yaw: {}, pitch: {}, roll: {}", self.throttle, self.yaw, self.pitch, self.roll)
    }
}