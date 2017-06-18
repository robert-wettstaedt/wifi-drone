extern crate glutin;

use controls::command::*;

use self::glutin::*;

pub struct WindowManager {
    pub window: Window,
    events_loop: EventsLoop
}

impl WindowManager {
    pub fn new() -> WindowManager {
        let events_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(800, 600)
            .build(&events_loop)
            .unwrap();

        let _ = unsafe {
            window.make_current()
        };

        WindowManager { window: window, events_loop: events_loop }
    }

    pub fn get_pressed_keys(&self, cmd: &mut Command) {
        let events_loop = &self.events_loop;

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
                            VirtualKeyCode::Escape => cmd.mode = DroneMode::Abort,
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