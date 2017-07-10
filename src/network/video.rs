use constants;

use std::error::Error;
use std::thread;
use std::net::TcpStream;
use std::io::{Write, Read};
use std::fs::File;

pub struct Video {
    input_stream: TcpStream,
    output_stream: TcpStream,
    data: Vec<u8>,
}

impl Video {
    pub fn new() -> Video {
        let output_stream = match TcpStream::connect(format!("{}:{}", constants::FFMPEG_HOST, constants::FFMPEG_TCP_PORT)) {
            Ok(stream) => stream,
            Err(e) => panic!("Couldn't connect to video output socket: {}", e.description()),
        };

        let input_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
            Ok(stream) => stream,
            Err(e) => panic!("Couldn't connect to video input socket: {}", e.description()),
        };

        let data = constants::get_video_2();

        return Video { input_stream: input_stream, output_stream: output_stream, data: data };
    }

    pub fn start(self) {
        thread::spawn(move || self.start_async());
    }

    fn start_async(mut self) {
        match self.input_stream.write(self.data.as_slice()) {
            Ok(_) => debug!("Sent video 2"),
            Err(e) => panic!("Error writing video 2: {}", e.description()),
        }
        let mut save_file = None;
        match File::create("out/ohh.h264") {
            Ok(file) => save_file = Some(file),
            Err(e) => println!("Error creating file: {}", e.description()),
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
                match self.output_stream.write(&buffer[0..buffer_size]) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing video output socket: {:?}", e.description()),
                }

                if let Some(mut file) = save_file {
                    match file.write(&buffer[0..buffer_size]) {
                        Ok(_) => (),
                        Err(e) => println!("Error writing save file: {:?}", e.description()),
                    }
                    save_file = Some(file);
                }
            }
        }
    }
}