use std::error::Error;
use std::io::Read;
use std::fs::{self, File};
use std::env;
use std::path::Path;
use std::path::PathBuf;

pub static DRONE_HOST: &'static str = "172.16.10.1";
pub static DRONE_TCP_PORT: usize = 8888;
pub static DRONE_UDP_PORT: usize = 8895;

pub static FFPLAY_HOST: &'static str = "127.0.0.1";
pub static FFPLAY_TCP_PORT: usize = 8889;

static HANDSHAKE: &'static str = include_str!("../res/handshake.dat");
static GAMEPAD: &'static str = include_str!("../res/gamepad.dat");
static VIDEO_1_1: &'static str = include_str!("../res/video_1_1.dat");
static VIDEO_1_2: &'static str = include_str!("../res/video_1_2.dat");
static VIDEO_2: &'static str = include_str!("../res/video_2.dat");
static HEARTBEAT: &'static str = include_str!("../res/heartbeat.dat");

pub fn get_handshake() -> Vec<u8> {
    return read_data(HANDSHAKE);
}

pub fn get_gamepad() -> Vec<u8> {
    return read_data(GAMEPAD);
}

pub fn get_video_1_1() -> Vec<u8> {
    return read_data(VIDEO_1_1);
}

pub fn get_video_1_2() -> Vec<u8> {
    return read_data(VIDEO_1_2);
}

pub fn get_video_2() -> Vec<u8> {
    return read_data(VIDEO_2);
}

pub fn get_heartbeat() -> Vec<u8> {
    return read_data(HEARTBEAT);
}

fn read_data(data_str: &str) -> Vec<u8> {
    let mut data = Vec::new();

    for slice in data_str.split(|c| c == ' ' || c == '\n') {
        match u8::from_str_radix(&slice[..2], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", data_str, e.description()),
        }
        match u8::from_str_radix(&slice[2..], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", data_str, e.description()),
        }
    }

    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_valid() {
        let data = read_data(HANDSHAKE);
        assert_eq!(data.len(), 106);
    }

    #[test]
    #[should_panic]
    fn test_read_file_invalid() {
        read_data("invalid");
    }
}
