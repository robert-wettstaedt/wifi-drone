pub mod decoder;
pub mod renderer;

use constants;
use self::decoder::Decoder;
use self::renderer::Renderer;
use super::window_manager::WindowManager;

pub struct Video {
    renderer: Renderer,
    pub decoder: Decoder
}

impl Video {
    pub fn new(window_manager: WindowManager) -> Video {
        let path = format!("tcp://{}:{}?listen", constants::FFMPEG_HOST, constants::FFMPEG_TCP_PORT);

        let mut renderer = Renderer::new(window_manager);
        let mut decoder = Decoder::new(path.as_str());

        Video { renderer, decoder }
    }

    pub fn start(mut self) {
        self.decoder.start(&mut self.renderer);
    }
}