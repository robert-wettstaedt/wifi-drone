extern crate ffmpeg;

use super::Renderer;

use ffmpeg::*;
use std::thread;

pub struct Decoder {
    codec: decoder::Video,
    context: format::context::Input
}

impl Decoder {
    pub fn new(path: &str) -> Decoder {
        format::network::init();
        format::register_all();
        println!("format::register_all");

        let _path = path.to_owned();
        thread::spawn(move || {
            println!("{}", _path);
            let context = match format::input(&_path) {
                Ok(context) => context,
                Err(e) => panic!("Error opening h.264 stream with ffmpeg: {:?}", e),
            };
            println!("format::input");
        });
        println!("thread::spawn");
        let context = match format::input(&path) {
            Ok(context) => context,
            Err(e) => panic!("Error opening h.264 stream with ffmpeg: {:?}", e),
        };

        let codec: decoder::Video;
        {
            // Spawn the video decoder.
            let stream = context.streams().find(|s| s.codec().medium() == media::Type::Video);
            if stream.is_some() {
                match stream.unwrap().codec().decoder().video() {
                    Ok(_codec) => codec = _codec,
                    Err(e) => panic!("Error getting ffmpeg video decoder: {}", e),
                };
            } else {
                panic!("No video stream found");
            }
        }
        println!("context.streams");

        Decoder { context, codec }
    }

    pub fn start(&mut self, renderer: &mut Renderer) {
        let mut decoded   = frame::Video::empty();
        let mut converter = self.codec.converter(format::Pixel::RGBA).unwrap();
        let mut index = 0;

        println!("{}:{}", self.codec.width(), self.codec.height());

        for (_, packet) in self.context.packets() {
            match self.codec.decode(&packet, &mut decoded) {
                Ok(true) => {
                    let mut frame = frame::Video::empty();
                    frame.clone_from(&decoded);
                    converter.run(&decoded, &mut frame).unwrap();

                    let buf: &[u8] = frame.data(0);
                    renderer.render(buf, self.codec.width(), self.codec.height());
                },
                Ok(false) => println!("Error false"),
                Err(ffmpeg::Error::Eof) => println!("Error::Eof"),
                Err(error) => panic!("Error decoding packet: {:?}", error),
            }

            index = index + 1;
        }
    }
}