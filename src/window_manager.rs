use glutin::*;

use std::sync::mpsc::Sender;
use std::time::SystemTime;

static UPDATE_INTERVAL: u64 = 50;

pub struct WindowManager {
    pub window: Window,
    events_loop: EventsLoop,
    keypress_tx: Sender<(ElementState, VirtualKeyCode)>,
    last_update: SystemTime,
}

impl WindowManager {
    pub fn new(keypress_tx: Sender<(ElementState, VirtualKeyCode)>) -> WindowManager {
        let events_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(800, 600)
            .build(&events_loop)
            .unwrap();

        WindowManager { window, events_loop, keypress_tx, last_update: SystemTime::now() }
    }

    pub fn update_pressed_keys(&self) {
        let events_loop = &self.events_loop;

        match self.last_update.elapsed() {
            Ok(elapsed) => {
                let ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
                if ms < UPDATE_INTERVAL {
                    return ();
                }
            },
            Err(e) => debug!("Could not retrieve elapsed system time: {}", e),
        }

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput(state, _, virt_key_code, _) => {
                        match self.keypress_tx.send((state, virt_key_code.unwrap())) {
                            Ok(_) => (),
                            Err(e) => debug!("Could not send keypress_tx: {}", e),
                        }
                    },

                    WindowEvent::Closed => events_loop.interrupt(),

                    _ => (),
                },
            }
        });
    }
}