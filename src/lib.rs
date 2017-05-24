#[macro_use] extern crate log;
extern crate env_logger;

use std::error::Error;

use std::{thread, time};
use std::net::{TcpStream, UdpSocket};
use std::io::{Write, Read};
use std::fs::File;
use std::process::{Command, Stdio};

pub static DRONE_HOST: &'static str = "172.16.10.1";
pub static DRONE_TCP_PORT: usize = 8888;
pub static DRONE_UDP_PORT: usize = 8895;

pub static FFPLAY_HOST: &'static str = "127.0.0.1";
pub static FFPLAY_TCP_PORT: usize = 8889;

pub fn connect() {
    env_logger::init().unwrap();

    start_ffplay();

    let mut handshake_stream = match TcpStream::connect(format!("{}:{}", DRONE_HOST, DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Error connecting to handshake socket: {}", e.description()),
    };
    let files = ["handshake", "video_1_1", "video_1_2"];
    for file in files.into_iter() {
        let path = format!("res/{}.dat", file).to_string();
        match handshake_stream.write(read_file(&path).as_slice()) {
            Ok(_) => debug!("Sent {}", file),
            Err(e) => panic!("Error writing {}: {}", file, e.description()),
        }
    }

    let gamepad_socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(e) => panic!("Error connecting to gamepad socket: {}", e.description()),
    };
    match gamepad_socket.send_to(read_file("res/gamepad.dat").as_slice(), format!("{}:{}", DRONE_HOST, DRONE_UDP_PORT)) {
        Ok(_) => debug!("Sent gamepad"),
        Err(e) => panic!("Error writing gamepad: {}", e.description()),
    }

    thread::spawn(move || start_heartbeat());

    start_video();
}

fn start_ffplay() {
    thread::spawn(move || {
        Command::new("ffplay")
            .arg("-f")
            .arg("h264")
            .arg("-codec:v")
            .arg("h264")
            .arg(format!("tcp://{}:{}?listen", FFPLAY_HOST, FFPLAY_TCP_PORT))
            .output();
    });

    let interval = time::Duration::from_secs(5);
    thread::sleep(interval);
}

fn start_video() {
    debug!("Starting video");

    let mut input_stream = match TcpStream::connect(format!("{}:{}", DRONE_HOST, DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Couldn't connect to video input socket: {}", e.description()),
    };

    let mut output_stream = match TcpStream::connect(format!("{}:{}", FFPLAY_HOST, FFPLAY_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Couldn't connect to video output socket: {}", e.description()),
    };

    match input_stream.write(read_file("res/video_2.dat").as_slice()) {
        Ok(_) => debug!("Sent video 2"),
        Err(e) => panic!("Error writing video 2: {}", e.description()),
    }

    loop {
        let mut buffer = [0; 8192];
        let mut buffer_size = 0;

        match input_stream.take_error() {
            Ok(_) => (),
            Err(e) => println!("Error on video input socket: {:?}", e.description()),
        }

        match output_stream.take_error() {
            Ok(_) => (),
            Err(e) => println!("Error on video output socket: {:?}", e.description()),
        }

        match input_stream.read(&mut buffer[..]) {
            Ok(size) => buffer_size = size,
            Err(e) => println!("Error reading video input socket: {:?}", e.description()),
        }

        if buffer_size > 0 && buffer_size != 106 {
            debug!("Buffer size video input socket: {:?}", buffer_size);

            match output_stream.write(&buffer[0..buffer_size]) {
                Ok(_) => (),
                Err(e) => println!("Error writing video output socket: {:?}", e.description()),
            }
        }
    }
}

fn start_heartbeat() {
    let mut stream = match TcpStream::connect(format!("{}:{}", DRONE_HOST, DRONE_TCP_PORT)) {
        Ok(stream) => stream,
        Err(e) => panic!("Error connecting to heartbeat socket: {}", e.description()),
    };

    let heartbeat_data = read_file("res/heartbeat.dat");
    let heartbeat_data_slice = heartbeat_data.as_slice();

    let interval = time::Duration::from_secs(5);

    loop {
        thread::sleep(interval);

        let mut buffer = [0; 8192];
        let mut buffer_size = 0;

        match stream.take_error() {
            Ok(_) => (),
            Err(e) => println!("Error on heartbeat socket: {:?}", e.description()),
        }

        match stream.write(heartbeat_data_slice) {
            Ok(_) => (),
            Err(e) => println!("Error writing heartbeat socket: {:?}", e.description()),
        }

        match stream.read(&mut buffer[..]) {
            Ok(size) => buffer_size = size,
            Err(e) => println!("Error reading heartbeat socket: {:?}", e.description()),
        }

        debug!("Received heartbeat with length: {}", buffer_size);
    }
}

fn read_file(path: &str) -> Vec<u8> {
    let mut buffer = String::new();
    let mut data = Vec::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", path, e.description()),
    };

    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't read {}: {}", path, e.description()),
    };

    for slice in buffer.split(|c| c == ' ' || c == '\n') {
        match u8::from_str_radix(&slice[..2], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", path, e.description()),
        }
        match u8::from_str_radix(&slice[2..], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", path, e.description()),
        }
    }

    return data;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_valid() {
        let data = read_file("res/handshake.dat");
        assert_eq!(data.len(), 106);
    }

    #[test]
    #[should_panic]
    fn test_read_file_invalid() {
        read_file("res/invalid_file.dat");
    }

    #[test]
    fn test_connect_valid() {
        connect();
    }
}
