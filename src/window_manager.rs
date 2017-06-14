extern crate glutin;

use self::glutin::*;
use ffmpeg::*;

pub struct WindowManager {
    events_loop: EventsLoop,
    buffer: Option<BufferView<[(u8, u8, u8, u8)]>>,
    texture: Option<SrgbTexture2d>,
}

impl WindowManager {
    pub fn new(display: &Display) -> WindowManager {
        WindowManager { events_loop: EventsLoop::new(), buffer: None, texture: None }
    }

    pub fn start(&mut self) {
        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(720, 576)
            .build(&(self.events_loop))
            .unwrap();

        let _ = unsafe {
            window.make_current()
        };
    }

    pub fn render(mut self, frame: &frame::Video) {
        if self.buffer.is_none() {
            self.buffer = Some(BufferView::empty_array(
                self.display,
                PixelUnpackBuffer,
                (frame.width() * frame.height()) as usize,
                Persistent).unwrap()
            );

            self.texture = Some(SrgbTexture2d::empty_with_format(
                self.display,
                U8U8U8U8,
                NoMipmap,
                frame.width(),
                frame.height()).unwrap()
            );
        }
    }
}