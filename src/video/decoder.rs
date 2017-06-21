extern crate ffmpeg;

use super::Renderer;

use self::ffmpeg::*;

use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Sender;

pub struct Decoder {
    codec: decoder::Video,
    context: format::context::Input,
}

pub fn input_with(path: &str) -> Result<format::context::Input, Error> {
    let mut options = Dictionary::new();
    options.set("iformat", "h264");
    options.set("video_codec", "h264");

    unsafe {
        let format = CString::new("h264").unwrap();
        let fmt = sys::av_find_input_format(format.as_ptr());

        let mut ps   = ptr::null_mut();
        let     path = CString::new(path).unwrap();
        let mut opts = options.disown();

        println!("input_with");

        let     res  = sys::avformat_open_input(&mut ps, path.as_ptr(), fmt, &mut opts);
        println!("sys::avformat_open_input");

        Dictionary::own(opts);

        match res {
            0 => {
                match sys::avformat_find_stream_info(ps, ptr::null_mut()) {
                    r if r >= 0 => Ok(format::context::Input::wrap(ps)),
                    e           => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e))
        }
    }
}

impl Decoder {
    pub fn new(path: &str, decoder_tx: Sender<()>) -> Decoder {
        format::network::init();
        format::register_all();

        decoder_tx.send(()).unwrap();

        let context = match input_with(&path) {
            Ok(context) => context,
            Err(e) => panic!("Error opening h.264 stream with ffmpeg: {:?}", e),
        };

        let codec: decoder::Video;
        {
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

        Decoder { context, codec }
    }

    pub fn start(&mut self, renderer: &mut Renderer) {
        let mut decoded   = frame::Video::empty();
        let mut converter = self.codec.converter(format::Pixel::RGBA).unwrap();
        let mut index = 0;

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