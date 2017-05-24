use constants;

use std::error::Error;

use std::{thread, time};
use std::net::TcpStream;
use std::io::{Write, Read};
use std::process::{Command};

pub struct Video {
    input_stream: TcpStream,
    output_stream: TcpStream,
    data: Vec<u8>,
}

impl Video {
    pub fn new() -> Video {
        let input_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
            Ok(stream) => stream,
            Err(e) => panic!("Couldn't connect to video input socket: {}", e.description()),
        };

        Video::start_ffplay();

        let output_stream = match TcpStream::connect(format!("{}:{}", constants::FFPLAY_HOST, constants::FFPLAY_TCP_PORT)) {
            Ok(stream) => stream,
            Err(e) => panic!("Couldn't connect to video output socket: {}", e.description()),
        };

        let data = constants::read_file("res/video_2.dat");

        return Video { input_stream: input_stream, output_stream: output_stream, data: data };
    }

    pub fn start(self) {
        self.start_streaming();
    }

    fn start_ffplay() {
        thread::spawn(move || {
            match Command::new("ffplay")
                .arg("-f")
                .arg("h264")
                .arg("-codec:v")
                .arg("h264")
                .arg(format!("tcp://{}:{}?listen", constants::FFPLAY_HOST, constants::FFPLAY_TCP_PORT))
                .output() {
                Ok(_) => (),
                Err(e) => panic!("Couldn't start ffplay: {}", e.description()),
            };
        });

        let interval = time::Duration::from_secs(5);
        thread::sleep(interval);
    }

    fn start_streaming(mut self) {
        match self.input_stream.write(self.data.as_slice()) {
            Ok(_) => debug!("Sent video 2"),
            Err(e) => panic!("Error writing video 2: {}", e.description()),
        }
        let mut buffer = [0; 8192];
        let mut buffer_size = 0;

        loop {
            match self.input_stream.take_error() {
                Ok(_) => (),
                Err(e) => println!("Error on video input socket: {:?}", e.description()),
            }

            match self.output_stream.take_error() {
                Ok(_) => (),
                Err(e) => println!("Error on video output socket: {:?}", e.description()),
            }

            match self.input_stream.read(&mut buffer[..]) {
                Ok(size) => buffer_size = size,
                Err(e) => println!("Error reading video input socket: {:?}", e.description()),
            }

            if buffer_size > 0 && buffer_size != 106 {
                debug!("Buffer size video input socket: {:?}", buffer_size);

                match self.output_stream.write(&buffer[0..buffer_size]) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing video output socket: {:?}", e.description()),
                }
            }
        }
    }
}