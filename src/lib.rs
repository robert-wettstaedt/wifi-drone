#[macro_use] extern crate log;
extern crate env_logger;
extern crate glutin;

pub mod constants;
pub mod controls;
pub mod network;
pub mod video;
pub mod window_manager;

use glutin::{ElementState, VirtualKeyCode};
use window_manager::WindowManager;
use video::Video;

use std::sync::mpsc::*;

pub fn connect(listener: video::VideoListener) {
    env_logger::init().unwrap();

    let (keypress_tx, keypress_rx): (Sender<(ElementState, VirtualKeyCode)>, Receiver<(ElementState, VirtualKeyCode)>) = channel();

    let window_manager = WindowManager::new(keypress_tx);
    let video = Video::new(&window_manager);
    network::start(keypress_rx);
    video.render_video();
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_valid() {
        connect(video::VideoListener::new(cb));
    }

     fn cb(data: &mut [u8], width: u32, height: u32) {
     }
}
