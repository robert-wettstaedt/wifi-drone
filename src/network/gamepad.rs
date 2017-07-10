use constants;
use controls::command::{Command, DroneMode};
use glutin::{ElementState, VirtualKeyCode};

use std::error::Error;
use std::thread;
use std::net::UdpSocket;
use std::sync::mpsc::Receiver;

pub type Callback = fn(command: &mut Command);

pub struct CommandListener {
    pub callback: Callback
}

impl CommandListener {
    pub fn new(callback: Callback) -> CommandListener {
        CommandListener { callback }
    }
}

pub struct Gamepad  {
    socket: UdpSocket,
    keypress_rx: Receiver<(ElementState, VirtualKeyCode)>,
    command_listener: CommandListener
}

impl Gamepad {
    pub fn new(keypress_rx: Receiver<(ElementState, VirtualKeyCode)>, command_listener: CommandListener) -> Gamepad {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(e) => panic!("Error connecting to gamepad socket: {}", e.description()),
        };

        return Gamepad { socket, keypress_rx, command_listener };
    }

    pub fn start(self) {
        match thread::Builder::new().name("network::gamepad".to_string()).spawn(move || self.start_async()) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    fn start_async(mut self) {
        let mut cmd = Command::new();

        loop {
            let (state, virt_key_code) = self.keypress_rx.recv().unwrap();

            let value = if state == ElementState::Pressed { 127 } else { 0 };

            match virt_key_code {
                VirtualKeyCode::Up => cmd.throttle = value,
                VirtualKeyCode::Down => cmd.throttle = -value,
                VirtualKeyCode::Right => cmd.yaw = value,
                VirtualKeyCode::Left => cmd.yaw = -value,
                VirtualKeyCode::W => cmd.pitch = value,
                VirtualKeyCode::S => cmd.pitch = -value,
                VirtualKeyCode::D => cmd.roll = value,
                VirtualKeyCode::A => cmd.roll = -value,
                VirtualKeyCode::Space => cmd.toggle_mode(state == ElementState::Pressed),
                VirtualKeyCode::Escape => cmd.mode = DroneMode::Abort,
                _ => (),
            }

            (self.command_listener.callback)(&mut cmd);
            println!("{:?}", cmd);

            self.write(&mut cmd);
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