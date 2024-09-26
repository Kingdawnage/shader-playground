use glfw::*;
use gl::*;
use types::GLuint;

mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, _events) = glfw
        .create_window(800, 600, "App", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);

        BindVertexArray(vao);
        BindBuffer(ARRAY_BUFFER, vbo);

        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            STATIC_DRAW,
        );

        VertexAttribPointer(
            0,
            3,
            FLOAT,
            FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        EnableVertexAttribArray(0);

        BindBuffer(ARRAY_BUFFER, 0);
        BindVertexArray(0);        
    }

    let shader = shader::Shader::new("src/shaders/shader.vert", "src/shaders/shader.frag");

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_program();

            BindVertexArray(vao);
            DrawArrays(TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
