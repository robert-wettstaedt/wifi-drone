use std::error::Error;
use std::io::Read;
use std::fs::File;

pub static DRONE_HOST: &'static str = "172.16.10.1";
pub static DRONE_TCP_PORT: usize = 8888;
pub static DRONE_UDP_PORT: usize = 8895;

pub static FFPLAY_HOST: &'static str = "127.0.0.1";
pub static FFPLAY_TCP_PORT: usize = 8889;

pub fn read_file(path: &str) -> Vec<u8> {
    let mut buffer = String::new();
    let mut data = Vec::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", path, e.description()),
    };

    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't read {}: {}", path, e.description()),
    };

    for slice in buffer.split(|c| c == ' ' || c == '\n') {
        match u8::from_str_radix(&slice[..2], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", path, e.description()),
        }
        match u8::from_str_radix(&slice[2..], 16) {
            Ok(v) => data.push(v),
            Err(e) => panic!("Couldn't parse {}: {}", path, e.description()),
        }
    }

    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_valid() {
        let data = read_file("res/handshake.dat");
        assert_eq!(data.len(), 106);
    }

    #[test]
    #[should_panic]
    fn test_read_file_invalid() {
        read_file("res/invalid_file.dat");
    }
}
