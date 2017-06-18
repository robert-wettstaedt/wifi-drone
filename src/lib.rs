#[macro_use] extern crate log;
extern crate env_logger;
extern crate ffmpeg;
extern crate glutin;

mod command;
mod constants;
mod gamepad;
mod heartbeat;
mod keyboard;
mod video;
mod window_manager;

use gamepad::Gamepad;
use heartbeat::Heartbeat;
use video::Video;
use window_manager::WindowManager;

use std::error::Error;
use std::net::TcpStream;
use std::io::Write;
use std::thread;

use ffmpeg::*;
use std::{ptr, env};
use std::ffi::{CString, CStr};
use std::path::{Path, PathBuf};
use std::fs::{self, File};

fn spawn (codec: &mut decoder::Video, context: &mut format::context::Input, wm: &mut WindowManager) {
    let mut decoded   = frame::Video::empty();
    let mut converter = codec.converter(format::Pixel::RGBA).unwrap();
    let mut index = 0;

    for (_, packet) in context.packets() {
        match codec.decode(&packet, &mut decoded) {
            Ok(true) => {
                let mut frame = frame::Video::empty();
                frame.clone_from(&decoded);
                converter.run(&decoded, &mut frame).unwrap();

                let buf: &[u8] = frame.data(0);
                wm.render(buf, codec.width(), codec.height());
            },
            Ok(false) => println!("Error false"),
            Err(ffmpeg::Error::Eof) => println!("Error::Eof"),
            Err(error) => panic!("Error decoding packet: {:?}", error),
        }

        index = index + 1;
    }
}

pub fn connect() {
    env_logger::init().unwrap();

    let mut wm = WindowManager::new();
    wm.start();

    let path: &str = "out/data.h264";
    let _path = path.to_owned();

    match std::fs::File::open(path) {
        Ok(file) => println!("{:?}", file),
        Err(e) => panic!("Error opening h264 file: {}", e.description()),
    }

    format::register_all();
    let mut context = match format::input(&_path) {
        Ok(context) => context,
        Err(e) => panic!("Error opening h264 file: {:?}", e),
    };

    let mut codec: decoder::Video;
    {
        // Spawn the video decoder.
        let stream = context.streams().find(|s| s.codec().medium() == media::Type::Video);
        if stream.is_some() {
            match stream.unwrap().codec().decoder().video() {
                Ok(_codec) => codec = _codec,
                Err(e) => panic!("Error getting video decoder: {}", e.description()),
            };
        } else {
            panic!("No video stream found");
        }
    }
    spawn(&mut codec, &mut context, &mut wm);
    println!("Done");

    loop {

    }







//    let mut handshake_stream = match TcpStream::connect(format!("{}:{}", constants::DRONE_HOST, constants::DRONE_TCP_PORT)) {
//        Ok(stream) => stream,
//        Err(e) => panic!("Error connecting to handshake socket: {}", e.description()),
//    };
//
//    let functions = [constants::get_handshake, constants::get_video_1_1, constants::get_video_1_2];
//    for index in 0..functions.len() {
//        match handshake_stream.write(functions[index]().as_slice()) {
//            Ok(_) => debug!("Sent {}", index),
//            Err(e) => panic!("Error writing {}: {}", index, e.description()),
//        }
//    }
//
//    Heartbeat::new().start();
//    Video::new().start();
//    Gamepad::new().start();
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_valid() {
        connect();
    }
}
