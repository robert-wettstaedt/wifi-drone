use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DroneMode {
    Normal = 0,
    TakingOff = 1,
    Landing = 2,
    TookOff = 3, // Helper mode, doesn't exist on drone
    Abort = 4,
}

pub struct Command {
    pub pitch: i8,
    pub yaw: i8,
    pub roll: i8,
    pub throttle: i8,
    pub mode: DroneMode,
    pub as_array: [u8; 8],
}

impl Command {
    pub fn new() -> Command {
        Command { throttle: 0, yaw: 0, pitch: 0, roll: 0, mode: DroneMode::Normal, as_array: [0; 8] }
    }

    pub fn toggle_mode(&mut self, is_toggling: bool) {
        match self.mode {
            DroneMode::Normal    => if is_toggling { self.mode = DroneMode::TakingOff },
            DroneMode::TakingOff => if is_toggling { () } else { self.mode = DroneMode::TookOff },
            DroneMode::TookOff   => if is_toggling { self.mode = DroneMode::Landing },
            DroneMode::Landing   => if is_toggling { () } else { self.mode = DroneMode::Normal },
            DroneMode::Abort     => if is_toggling { self.mode = DroneMode::Normal },
        }
    }

    /**
    * as_array[0] = constant
    * as_array[1] = roll
    * as_array[2] = pitch
    * as_array[3] = throttle
    * as_array[4] = yaw
    * as_array[5] = mode
    * as_array[6] = checksum
    * as_array[7] = constant
    */
    pub fn update_array(&mut self) {
        self.as_array[0] = 0x66;

        let commands = [self.roll, self.pitch, self.throttle, self.yaw];
        for i in 0..commands.len() {
            if commands[i] >= 0 {
                self.as_array[i + 1] = (commands[i] as u8) + 127;
            } else {
                self.as_array[i + 1] = (commands[i] + 127) as u8;
            }
        }

        self.as_array[5] = if self.mode == DroneMode::TookOff { DroneMode::Normal as u8 } else { self.mode as u8 };
        self.as_array[6] = (self.as_array[1] ^ self.as_array[2] ^ self.as_array[3] ^ self.as_array[4] ^ self.as_array[5]) & 0xFF;

        self.as_array[7] = 0x99;
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "throttle: {}, yaw: {}, pitch: {}, roll: {}, mode: {:?}", self.throttle, self.yaw, self.pitch, self.roll, self.mode)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throttle_up() {
        let mut cmd = Command::new();

        cmd.throttle = 127;
        cmd.update_array();
        assert_eq!(cmd.as_array[3], 254);
    }

    #[test]
    fn test_throttle_down() {
        let mut cmd = Command::new();

        cmd.throttle = -127;
        cmd.update_array();
        assert_eq!(cmd.as_array[3], 0);
    }
}