extern crate glutin;

use glutin::*;

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
}