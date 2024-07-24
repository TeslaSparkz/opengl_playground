extern crate gl;
extern crate glfw;
extern crate nalgebra as na;

use glfw::{Action, Context, Key, WindowEvent};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

fn read_shader_src(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open shader file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Failed to read shader file");
    src
}

fn compile_shader(src: &str, ty: u32) -> u32 {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut error = Vec::with_capacity(len as usize);
            error.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_mut_ptr() as *mut gl::types::GLchar);
            panic!("{}", String::from_utf8(error).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: u32, fs: u32) -> u32 {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut error = Vec::with_capacity(len as usize);
            error.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_mut_ptr() as *mut gl::types::GLchar);
            panic!("{}", String::from_utf8(error).expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Set OpenGL version
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(800, 600, "OpenGL Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Read shader sources from files
    let vertex_shader_src = read_shader_src("shaders/vertex_shader.glsl");
    let fragment_shader_src = read_shader_src("shaders/fragment_shader.glsl");

    // Compile shaders
    let vs = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
    let fs = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vs, fs);

    // Vertex data
    let vertices: [f32; 9] = [
        0.0,  0.5, 0.0,
       -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
    ];

    // Generate and bind Vertex Array Object (VAO) and Vertex Buffer Object (VBO)
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as gl::types::GLint, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Main loop
    while !window.should_close() {
        // Poll events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use shader program
            gl::UseProgram(shader_program);

            // Draw the triangle
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
}
