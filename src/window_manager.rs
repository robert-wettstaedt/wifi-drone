extern crate glutin;

use std::ffi::CStr;
use std::{mem, ptr};
use glutin::*;
use ffmpeg::*;
use std::{thread, time};

mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/test_gl_bindings.rs"));
}

pub struct WindowManager {
    window: Window,
    events_loop: EventsLoop,
    gl: gl::Gl
}

impl WindowManager {
    pub fn new() -> WindowManager {
        let events_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title("FPV")
            .with_dimensions(720, 576)
            .build(&events_loop)
            .unwrap();

        let _ = unsafe {
            window.make_current()
        };

        let gl = gl::Gl::load_with(|ptr| (&window).get_proc_address(ptr) as *const _);

        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _).to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };

        println!("OpenGL version {}", version);

//        const WIDTH: i32 = 4;
//        const HEIGHT: i32 = 4;
//        let pixels: [u8; (WIDTH * HEIGHT * 4) as usize] = [
//            255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,
//              0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,
//            255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,
//              0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255
//        ];

        const WIDTH: i32 = 720;
        const HEIGHT: i32 = 576;
        let pixels: [u8; (WIDTH * HEIGHT * 4) as usize] = include!("../out/data.txt");

        println!("pixels - length: {} - [{}, {}, {}, {}, ...]", pixels.len(), pixels[0], pixels[1], pixels[2], pixels[3]);

        let vertices: [f32; 28] = [
            //  Position      Color    Texcoords
            -1.0,  1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
             1.0,  1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
             1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // Bottom-right
            -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0  // Bottom-left
        ];

        let elements = [
            0, 1, 2,
            2, 3, 0
        ];

        unsafe {
            // Create Vertex Array Object
            let mut vao = mem::uninitialized();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            let mut vbo = mem::uninitialized();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl.BufferData(gl::ARRAY_BUFFER,
                          (vertices.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                          vertices.as_ptr() as *const _,
                          gl::STATIC_DRAW);

            // Create an element array
            let mut ebo = mem::uninitialized();
            gl.GenBuffers(1, &mut ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                          (elements.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                          elements.as_ptr() as *const _,
                          gl::STATIC_DRAW);

            // Create and compile the vertex shader
            let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vertex_shader, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl.CompileShader(vertex_shader);

            // Create and compile the fragment shader
            let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fragment_shader, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl.CompileShader(fragment_shader);

            // Link the vertex and fragment shader into a shader program
            let shader_program = gl.CreateProgram();
            gl.AttachShader(shader_program, vertex_shader);
            gl.AttachShader(shader_program, fragment_shader);
            gl.LinkProgram(shader_program);
            gl.UseProgram(shader_program);

            // Specify the layout of the vertex data
            let pos_attrib = gl.GetAttribLocation(shader_program, b"position\0".as_ptr() as *const _);
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, gl::FALSE,
                                   7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                   ptr::null());

            let col_attrib = gl.GetAttribLocation(shader_program, b"color\0".as_ptr() as *const _);
            gl.EnableVertexAttribArray(col_attrib as gl::types::GLuint);
            gl.VertexAttribPointer(col_attrib as gl::types::GLuint, 3, gl::FLOAT, gl::FALSE,
                                   7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                   (2 * mem::size_of::<f32>()) as *const () as *const _);

            let tex_attrib = gl.GetAttribLocation(shader_program, b"texcoord\0".as_ptr() as *const _);
            gl.EnableVertexAttribArray(tex_attrib as u32);
            gl.VertexAttribPointer(tex_attrib as u32, 2, gl::FLOAT, gl::FALSE,
                                   7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                   (5 * mem::size_of::<f32>()) as *const () as *const _);

            let mut textures = mem::uninitialized();
            gl.GenTextures(1, &mut textures);

            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, textures);
            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, WIDTH, HEIGHT, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const _);
            gl.Uniform1i(gl.GetUniformLocation(shader_program, b"tex\0".as_ptr() as *const _), 0);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        WindowManager { window: window, events_loop: events_loop, gl: gl }
    }

    pub fn start(&mut self) {
        self.start_input_listener();
    }

    fn start_input_listener(&mut self) {
        loop {
            let events_loop = &mut self.events_loop;

            unsafe {
                // Clear the screen to black
                self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
                self.gl.Clear(self::gl::COLOR_BUFFER_BIT);

                // Draw a rectangle from the 2 triangles using 6 indices
                self.gl.DrawElements(self::gl::TRIANGLES, 6, self::gl::UNSIGNED_INT, ptr::null());
            }
            self.window.swap_buffers();

            events_loop.poll_events(|event| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::KeyboardInput(state, _, virt_key_code, _) => {

                            let value = if state == ElementState::Pressed { 127 } else { 0 };

                            match virt_key_code.unwrap() {
                                _ => (),
                            }
                        },

                        WindowEvent::Closed => events_loop.interrupt(),

                        _ => (),
                    },
                }
            });

//            let interval = time::Duration::from_millis(50);
//            thread::sleep(interval);
        }
    }
}

static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,     0.0,  0.0,  0.0,
     0.5,  0.0,  1.0,     0.0,  0.5, -0.5,
     0.0,  0.0,  1.0
];

const VS_SRC: &'static [u8] = b"
    #version 150 core

    in vec2 position;
    in vec3 color;
    in vec2 texcoord;

    out vec3 Color;
    out vec2 Texcoord;

    void main() {
        Color = color;
        Texcoord = texcoord;
        gl_Position = vec4(position, 0.0, 1.0);
    }
\0";

const FS_SRC: &'static [u8] = b"
    #version 150 core

    uniform sampler2D tex;

    in vec3 Color;
    in vec2 Texcoord;

    out vec4 outColor;

    void main() {
        outColor = texture(tex, Texcoord);
    }
\0";