use constants;
use controls::command::{Command, DroneMode};
use glutin::{ElementState, VirtualKeyCode};

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::net::UdpSocket;
use std::sync::mpsc::Receiver;

static UPDATE_INTERVAL: u64 = 50;

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
    keypress_rx: Receiver<Vec<VirtualKeyCode>>,
    command_listener: CommandListener
}

impl Gamepad {
    pub fn new(keypress_rx: Receiver<Vec<VirtualKeyCode>>, command_listener: CommandListener) -> Gamepad {
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
        let mut last_pressed_keys: Vec<VirtualKeyCode> = vec!();

        loop {
            cmd.reset_directions();

            let pressed_keys = self.keypress_rx.recv().unwrap();
            let pressed_keys_clone = pressed_keys.clone();
            print!("{:?}\t", pressed_keys);

            for pressed_key in pressed_keys {
                let value = 127;

                print!("{:?}  ", pressed_key);

                match pressed_key {
                    VirtualKeyCode::Up => cmd.throttle = value,
                    VirtualKeyCode::Down => cmd.throttle = -value,
                    VirtualKeyCode::Right => cmd.yaw = value,
                    VirtualKeyCode::Left => cmd.yaw = -value,
                    VirtualKeyCode::W => cmd.pitch = value,
                    VirtualKeyCode::S => cmd.pitch = -value,
                    VirtualKeyCode::D => cmd.roll = value,
                    VirtualKeyCode::A => cmd.roll = -value,
                    VirtualKeyCode::Return => self.take_off(!last_pressed_keys.contains(&pressed_key), &mut cmd),
                    VirtualKeyCode::Space => cmd.toggle_mode(!last_pressed_keys.contains(&pressed_key)),
                    VirtualKeyCode::Escape => cmd.mode = DroneMode::Abort,
                    _ => (),
                }
            }
            (self.command_listener.callback)(&mut cmd);

            println!("\n{:?}\n", cmd);

            self.write(&mut cmd);
            last_pressed_keys = pressed_keys_clone;
        }
    }

    fn write(&mut self, cmd: &mut Command) {
        cmd.update_array();
        match self.socket.send_to(&(cmd.as_array), format!("{}:{}", constants::DRONE_HOST, constants::DRONE_UDP_PORT)) {
            Ok(_) => debug!("Sent gamepad data"),
            Err(e) => panic!("Error writing gamepad data: {}", e.description()),
        }
    }

    fn take_off(&mut self, is_toggling: bool, mut cmd: &mut Command) {
        if !is_toggling {
            return;
        }

        let duration = Duration::from_millis(UPDATE_INTERVAL);

        cmd.mode = DroneMode::Normal;
        self.write(&mut cmd);
        thread::sleep(duration);
        cmd.mode = DroneMode::TakingOff;

        for i in 0..50 {
            print!("{}  ", i);
            self.write(&mut cmd);
            thread::sleep(duration);
            cmd.throttle = (10 + i) as i8;

            (self.command_listener.callback)(&mut cmd);
        }

        thread::sleep(duration);
        cmd.mode = DroneMode::TookOff;
        self.write(&mut cmd);
    }
}