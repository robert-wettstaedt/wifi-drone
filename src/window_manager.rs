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

        WindowManager { window: window, events_loop: events_loop, gl: gl }
    }

    pub fn start(&mut self) {
        let version = unsafe {
            let data = CStr::from_ptr(self.gl.GetString(gl::VERSION) as *const _).to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };

        println!("OpenGL version {}", version);

        const WIDTH: i32 = 4;
        const HEIGHT: i32 = 4;
        let pixels: [u8; (WIDTH * HEIGHT * 4) as usize] = [
            255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,
              0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,
            255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,
              0, 255, 255, 255,   255,   0,   0, 255,     0, 255, 255, 255,   255,   0,   0, 255
        ];

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
            self.gl.GenVertexArrays(1, &mut vao);
            self.gl.BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            let mut vbo = mem::uninitialized();
            self.gl.GenBuffers(1, &mut vbo);
            self.gl.BindBuffer(self::gl::ARRAY_BUFFER, vbo);

            self.gl.BufferData(self::gl::ARRAY_BUFFER,
                               (vertices.len() * mem::size_of::<f32>()) as self::gl::types::GLsizeiptr,
                               vertices.as_ptr() as *const _,
                               self::gl::STATIC_DRAW);

            // Create an element array
            let mut ebo = mem::uninitialized();
            self.gl.GenBuffers(1, &mut ebo);
            self.gl.BindBuffer(self::gl::ELEMENT_ARRAY_BUFFER, ebo);

            self.gl.BufferData(self::gl::ELEMENT_ARRAY_BUFFER,
                               (elements.len() * mem::size_of::<u32>()) as self::gl::types::GLsizeiptr,
                               elements.as_ptr() as *const _,
                               self::gl::STATIC_DRAW);

            // Create and compile the vertex shader
            let vertex_shader = self.gl.CreateShader(self::gl::VERTEX_SHADER);
            self.gl.ShaderSource(vertex_shader, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            self.gl.CompileShader(vertex_shader);

            // Create and compile the fragment shader
            let fragment_shader = self.gl.CreateShader(self::gl::FRAGMENT_SHADER);
            self.gl.ShaderSource(fragment_shader, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            self.gl.CompileShader(fragment_shader);

            // Link the vertex and fragment shader into a shader program
            let shader_program = self.gl.CreateProgram();
            self.gl.AttachShader(shader_program, vertex_shader);
            self.gl.AttachShader(shader_program, fragment_shader);
            self.gl.LinkProgram(shader_program);
            self.gl.UseProgram(shader_program);

            // Specify the layout of the vertex data
            let pos_attrib = self.gl.GetAttribLocation(shader_program, b"position\0".as_ptr() as *const _);
            self.gl.EnableVertexAttribArray(pos_attrib as self::gl::types::GLuint);
            self.gl.VertexAttribPointer(pos_attrib as self::gl::types::GLuint, 2, self::gl::FLOAT, self::gl::FALSE,
                                        7 * mem::size_of::<f32>() as self::gl::types::GLsizei,
                                        ptr::null());

            let col_attrib = self.gl.GetAttribLocation(shader_program, b"color\0".as_ptr() as *const _);
            self.gl.EnableVertexAttribArray(col_attrib as self::gl::types::GLuint);
            self.gl.VertexAttribPointer(col_attrib as self::gl::types::GLuint, 3, self::gl::FLOAT, self::gl::FALSE,
                                        7 * mem::size_of::<f32>() as self::gl::types::GLsizei,
                                        (2 * mem::size_of::<f32>()) as *const () as *const _);

            let tex_attrib = self.gl.GetAttribLocation(shader_program, b"texcoord\0".as_ptr() as *const _);
            self.gl.EnableVertexAttribArray(tex_attrib as u32);
            self.gl.VertexAttribPointer(tex_attrib as u32, 2, self::gl::FLOAT, self::gl::FALSE,
                                        7 * mem::size_of::<f32>() as self::gl::types::GLsizei,
                                        (5 * mem::size_of::<f32>()) as *const () as *const _);

            let mut textures = mem::uninitialized();
            self.gl.GenTextures(1, &mut textures);

            self.gl.ActiveTexture(self::gl::TEXTURE0);
            self.gl.BindTexture(self::gl::TEXTURE_2D, textures);
            self.gl.Uniform1i(self.gl.GetUniformLocation(shader_program, b"tex\0".as_ptr() as *const _), 0);
        }

    }

    pub fn render(&mut self, buf: &[u8], width: u32, height: u32) {
        const WIDTH: i32 = 720;
        const HEIGHT: i32 = 576;

        unsafe {
            // Clear the screen to black
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl.Clear(self::gl::COLOR_BUFFER_BIT);

            self.gl.TexImage2D(self::gl::TEXTURE_2D, 0, self::gl::RGBA as i32, width as i32, height as i32, 0, self::gl::RGBA, self::gl::UNSIGNED_BYTE, buf.as_ptr() as *const _);
            self.gl.TexParameteri(self::gl::TEXTURE_2D, self::gl::TEXTURE_WRAP_S, self::gl::CLAMP_TO_EDGE as i32);
            self.gl.TexParameteri(self::gl::TEXTURE_2D, self::gl::TEXTURE_WRAP_T, self::gl::CLAMP_TO_EDGE as i32);
            self.gl.TexParameteri(self::gl::TEXTURE_2D, self::gl::TEXTURE_MIN_FILTER, self::gl::NEAREST as i32);
            self.gl.TexParameteri(self::gl::TEXTURE_2D, self::gl::TEXTURE_MAG_FILTER, self::gl::NEAREST as i32);

            // Draw a rectangle from the 2 triangles using 6 indices
            self.gl.DrawElements(self::gl::TRIANGLES, 6, self::gl::UNSIGNED_INT, ptr::null());
        }

        self.window.swap_buffers();

        let interval = time::Duration::from_millis(100);
        thread::sleep(interval);
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