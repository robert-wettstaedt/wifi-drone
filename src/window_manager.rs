use glutin::*;

use std::sync::mpsc::Sender;
use std::time::SystemTime;

static UPDATE_INTERVAL: u64 = 50;

pub struct WindowManager {
    pub window: Window,
    events_loop: EventsLoop,
    pressed_keys: Vec<VirtualKeyCode>,
    keypress_tx: Sender<Vec<VirtualKeyCode>>,
    last_update: SystemTime,
}

impl WindowManager {
    pub fn new(keypress_tx: Sender<Vec<VirtualKeyCode>>) -> WindowManager {
        let events_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(800, 600)
            .build(&events_loop)
            .unwrap();

        WindowManager { window, events_loop, keypress_tx, last_update: SystemTime::now(), pressed_keys: vec!() }
    }

    pub fn update_pressed_keys(&mut self) {
        let events_loop = &self.events_loop;

        let is_too_early = match self.last_update.elapsed() {
            Ok(elapsed) => {
                let ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
                ms < UPDATE_INTERVAL
            },
            Err(e) => true,
        };

        let mut state: ElementState = ElementState::Released;
        let mut virt_key_code: Option<VirtualKeyCode> = Some(VirtualKeyCode::Key0);

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput(_state, _, _virt_key_code, _) => {
                        state = _state;
                        virt_key_code = _virt_key_code;
                    },
                    WindowEvent::Closed => events_loop.interrupt(),
                    _ => (),
                },
            }
        });

        if virt_key_code.is_none() {
            return;
        }

        let key_code = virt_key_code.unwrap();
        let contains = self.pressed_keys.contains(&key_code);

        if state == ElementState::Pressed {
            if !contains {
                self.pressed_keys.push(key_code);
            }
        } else {
            match self.pressed_keys.iter().position(|code| *code == key_code) {
                Some(index) => self.pressed_keys.remove(index),
                None => VirtualKeyCode::Key0,
            };
        }

//        if is_too_early {
//            return;
//        }

        self.last_update = SystemTime::now();

        match self.keypress_tx.send(self.pressed_keys.clone()) {
            Ok(_) => (),
            Err(e) => debug!("Could not send keypress_tx: {}", e),
        }
    }
}