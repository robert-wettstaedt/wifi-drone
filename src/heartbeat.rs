use constants;

use std::error::Error;

use std::{thread, time};
use std::net::TcpStream;
use std::io::{Write, Read};

pub struct Heartbeat {
    stream: TcpStream,
    data: Vec<u8>,
}

impl Heartbeat {
    pub fn new() -> Heartbeat {
        let stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
            Ok(stream) => stream,
            Err(e) => panic!("Error connecting to heartbeat socket: {}", e.description()),
        };

        let data = constants::read_file("res/heartbeat.dat");

        return Heartbeat { stream: stream, data: data };
    }

    pub fn start(self) {
        thread::spawn(move || self.start_async());
    }

    fn start_async(mut self) {
        let interval = time::Duration::from_secs(5);
        let mut buffer = [0; 256];
        let mut buffer_size = 0;

        let data_slice = self.data.as_slice();

        loop {
            thread::sleep(interval);

            match self.stream.take_error() {
                Ok(_) => (),
                Err(e) => println!("Error on heartbeat socket: {:?}", e.description()),
            }

            match self.stream.write(data_slice) {
                Ok(_) => (),
                Err(e) => println!("Error writing heartbeat socket: {:?}", e.description()),
            }

            match self.stream.read(&mut buffer[..]) {
                Ok(size) => buffer_size = size,
                Err(e) => println!("Error reading heartbeat socket: {:?}", e.description()),
            }

            debug!("Received heartbeat with length: {}", buffer_size);
        }
    }
}