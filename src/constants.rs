use std::error::Error;

pub static DRONE_HOST: &'static str = "172.16.10.1";
pub static DRONE_TCP_PORT: usize = 8888;
pub static DRONE_UDP_PORT: usize = 8895;

pub static FFMPEG_HOST: &'static str = "127.0.0.1";
pub static FFMPEG_TCP_PORT: usize = 8889;

static HANDSHAKE_1: &'static str = include_str!("../res/handshake_1.dat");
static HANDSHAKE_2: &'static str = include_str!("../res/handshake_2.dat");
static HANDSHAKE_3: &'static str = include_str!("../res/handshake_3.dat");
static VIDEO: &'static str = include_str!("../res/video.dat");
static HEARTBEAT: &'static str = include_str!("../res/heartbeat.dat");

pub fn get_tcp_path() -> String {
    format!("tcp://{}:{}?listen", FFMPEG_HOST, FFMPEG_TCP_PORT)
}

pub fn get_handshake_1() -> Vec<u8> {
    return read_data(HANDSHAKE_1);
}

pub fn get_handshake_2() -> Vec<u8> {
    return read_data(HANDSHAKE_2);
}

pub fn get_handshake_3() -> Vec<u8> {
    return read_data(HANDSHAKE_3);
}

pub fn get_video() -> Vec<u8> {
    return read_data(VIDEO);
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
        let data = read_data(HANDSHAKE_1);
        assert_eq!(data.len(), 106);
    }

    #[test]
    #[should_panic]
    fn test_read_file_invalid() {
        read_data("invalid");
    }
}
