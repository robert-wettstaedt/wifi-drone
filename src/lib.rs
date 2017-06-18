#[macro_use] extern crate log;
extern crate env_logger;
extern crate ffmpeg;
extern crate glutin;

pub mod constants;
pub mod controls;
pub mod network;
pub mod video;
pub mod window_manager;

use window_manager::WindowManager;
use video::Video;

pub fn connect() {
    env_logger::init().unwrap();

    let window_manager = WindowManager::new();
    println!("WindowManager::new");
    let mut video = Video::new(window_manager);
    println!("Video::new");

    network::start();
    println!("network::start");
    controls::start();
    println!("controls::start");

    video.start();
    println!("video.start");
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_valid() {
        connect();
    }
}
