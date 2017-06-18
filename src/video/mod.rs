pub mod decoder;
pub mod renderer;

use constants;
use self::decoder::Decoder;
use self::renderer::Renderer;
use super::window_manager::WindowManager;

use std::thread;

pub struct Video <'a> {
    renderer: Renderer<'a>,
    decoder: Option<Decoder>,
    decoder_thread_handle: thread::JoinHandle<Decoder>
}

impl <'a> Video <'a> {
    pub fn new(window_manager: &WindowManager) -> Video {
        let path = format!("tcp://{}:{}?listen", constants::FFMPEG_HOST, constants::FFMPEG_TCP_PORT);
//        let path = format!("out/data.h264");

        let renderer = Renderer::new(&window_manager);
        let handle: thread::JoinHandle<Decoder> = thread::spawn(move || {
            Decoder::new(path.as_str())
        });

        Video { renderer, decoder: None, decoder_thread_handle: handle }
    }

    pub fn start(mut self) {
        println!("joining");
        match self.decoder_thread_handle.join() {
            Ok(decoder) => {
                self.decoder = Some(decoder);
                self.decoder.unwrap().start(&mut self.renderer);
            },
            Err(e) => println!("Error joining thread: {:?}", e),
        }
    }
}