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
use video::{Video, VideoListener};
use network::gamepad::CommandListener;
use controls::command::Command;

use std::sync::mpsc::*;

pub fn connect(path: String, video_listener: VideoListener, command_listener: CommandListener) {
    env_logger::init().unwrap();
    let _path = path.as_ref();

    let (keypress_tx, keypress_rx): (Sender<(ElementState, VirtualKeyCode)>, Receiver<(ElementState, VirtualKeyCode)>) = channel();

    let window_manager = WindowManager::new(keypress_tx);
    let video = Video::new(_path, &window_manager);
    if _path.starts_with("tcp:") {
        network::start(keypress_rx, command_listener);
    }
    video.render_video(video_listener);
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_connect_valid() {
        connect(constants::get_tcp_path(), VideoListener::new(video_callback), CommandListener::new(command_callback));
    }

    fn video_callback(data: &mut [u8], width: u32, height: u32) { }
    fn command_callback(command: &mut Command) { }
}
