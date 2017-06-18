pub mod command;
pub mod keyboard;

use self::command::Command;
use self::keyboard::Keyboard;

pub fn start() {
    Keyboard::new().start();
}