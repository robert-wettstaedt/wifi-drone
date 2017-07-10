extern crate glutin;
extern crate gl;

use window_manager::WindowManager;

use std::ffi::CStr;
use std::{mem, ptr};

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

const VERTICES: [f32; 28] = [
    //  Position      Color    Texcoords
    -1.0,  1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
    1.0,  1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
    1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // Bottom-right
    -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0  // Bottom-left
];

const ELEMENTS: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];

pub struct Renderer <'a> {
    window_manager: &'a WindowManager,
}

impl <'a> Renderer <'a> {
    pub fn new(window_manager: &WindowManager) -> Renderer {
        Renderer { window_manager: &window_manager }
    }

    pub fn prepare_gl(&mut self) {
        unsafe {
            let _ = self.window_manager.window.make_current();

            gl::load_with(|symbol| self.window_manager.window.get_proc_address(symbol) as *const _);

            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
            let version = String::from_utf8(data).unwrap();
            println!("OpenGL version {}", version);

            // Create Vertex Array Object
            let mut vao = mem::uninitialized();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            let mut vbo = mem::uninitialized();
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (VERTICES.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                           VERTICES.as_ptr() as *const _,
                           gl::STATIC_DRAW);

            // Create an element array
            let mut ebo = mem::uninitialized();
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (ELEMENTS.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                           ELEMENTS.as_ptr() as *const _,
                           gl::STATIC_DRAW);

            // Create and compile the vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // Create and compile the fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            // Link the vertex and fragment shader into a shader program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            gl::UseProgram(shader_program);

            // Specify the layout of the vertex data
            let pos_attrib = gl::GetAttribLocation(shader_program, b"position\0".as_ptr() as *const _);
            gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, gl::FALSE,
                                    7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                    ptr::null());

            let col_attrib = gl::GetAttribLocation(shader_program, b"color\0".as_ptr() as *const _);
            gl::EnableVertexAttribArray(col_attrib as gl::types::GLuint);
            gl::VertexAttribPointer(col_attrib as gl::types::GLuint, 3, gl::FLOAT, gl::FALSE,
                                    7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                    (2 * mem::size_of::<f32>()) as *const () as *const _);

            let tex_attrib = gl::GetAttribLocation(shader_program, b"texcoord\0".as_ptr() as *const _);
            gl::EnableVertexAttribArray(tex_attrib as u32);
            gl::VertexAttribPointer(tex_attrib as u32, 2, gl::FLOAT, gl::FALSE,
                                    7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                    (5 * mem::size_of::<f32>()) as *const () as *const _);

            let mut textures = mem::uninitialized();
            gl::GenTextures(1, &mut textures);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, textures);
            gl::Uniform1i(gl::GetUniformLocation(shader_program, b"tex\0".as_ptr() as *const _), 0);
        }
    }

    pub fn render(&mut self, buf: &[u8], width: u32, height: u32) {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, buf.as_ptr() as *const _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // Draw a rectangle from the 2 triangles using 6 indices
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        self.window_manager.update_pressed_keys();
        match self.window_manager.window.swap_buffers() {
            Ok(_) => (),
            Err(e) => println!("Error swapping buffers: {}", e),
        }
    }
}