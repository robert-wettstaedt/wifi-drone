extern crate glutin;

use command::Command;
use self::glutin::*;

pub struct Keyboard {
    events_loop: EventsLoop,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { events_loop: EventsLoop::new() }
    }

    pub fn start(&mut self) {
        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(1, 1)
            .build(&(self.events_loop))
            .unwrap();

        let _ = unsafe {
            window.make_current()
        };
    }

    pub fn get_pressed_keys(&mut self, cmd: &mut Command) {
        let events_loop = &mut self.events_loop;

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput(state, _, virt_key_code, _) => {

                        let value = if state == ElementState::Pressed { 127 } else { 0 };

                        match virt_key_code.unwrap() {
                            VirtualKeyCode::Up => cmd.throttle = value,
                            VirtualKeyCode::Down => cmd.throttle = -value,
                            VirtualKeyCode::Right => cmd.yaw = value,
                            VirtualKeyCode::Left => cmd.yaw = -value,
                            VirtualKeyCode::W => cmd.pitch = value,
                            VirtualKeyCode::S => cmd.pitch = -value,
                            VirtualKeyCode::D => cmd.roll = value,
                            VirtualKeyCode::A => cmd.roll = -value,
                            VirtualKeyCode::Space => cmd.toggle_mode(state == ElementState::Pressed),
                            _ => (),
                        }
                    },

                    WindowEvent::Closed => events_loop.interrupt(),

                    _ => (),
                },
            }
        });
    }
}