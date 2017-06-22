pub mod decoder;
pub mod renderer;

use constants;
use self::decoder::Decoder;
use self::renderer::Renderer;
use super::window_manager::WindowManager;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

pub type Callback = fn(data: &mut [u8], width: u32, height: u32);

pub struct VideoListener {
    pub callback: Callback
}

impl VideoListener {
    pub fn new(callback: Callback) -> VideoListener {
        VideoListener { callback }
    }
}

pub struct Video <'a> {
    renderer: Renderer<'a>,
    decoder: Option<Decoder>,
    decoder_thread_handle: thread::JoinHandle<Decoder>
}

impl <'a> Video <'a> {
    pub fn new(window_manager: &WindowManager) -> Video {
        let path = format!("tcp://{}:{}?listen", constants::FFMPEG_HOST, constants::FFMPEG_TCP_PORT);
//        let path = format!("out/data.h264");

        let (decoder_tx, decoder_rx): (Sender<()>, Receiver<()>) = channel();

        let renderer = Renderer::new(&window_manager);
        let handle: thread::JoinHandle<Decoder> = thread::spawn(move || {
            Decoder::new(path.as_str(), decoder_tx)
        });

        match decoder_rx.recv() {
            Ok(_) => (),
            Err(_) => thread::sleep(Duration::from_millis(500)),
        }

        Video { renderer, decoder: None, decoder_thread_handle: handle }
    }

    pub fn render_video(mut self, listener: VideoListener) {
        match self.decoder_thread_handle.join() {
            Ok(decoder) => {
                self.decoder = Some(decoder);
                self.decoder.unwrap().start(&mut self.renderer, listener);
            },
            Err(e) => println!("Error joining thread: {:?}", e),
        }
    }
}