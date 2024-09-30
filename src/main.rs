use core::panic;

use gl::*;
use glfw::*;
use stb_image::image::{self, LoadResult};
use types::GLuint;

mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, _events) = glfw
        .create_window(1000, 800, "App", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices: [f32; 20] = [
        // positions     // textures
        -0.5, -0.5, 0.0, 0.0, 0.0, 0.5, -0.5, 0.0, 1.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5,
        0.0, 1.0, 1.0,
    ];

    let indices: [u32; 6] = [0, 1, 2, 1, 2, 3];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut ebo: GLuint = 0;

    // Load image into texture
    let image_path = "src/assets/ecchi_rusalka.jpg";
    let image = match image::load(image_path) {
        LoadResult::ImageU8(image) => image,
        LoadResult::Error(msg) => panic!("Error loading image: {}", msg),
        _ => panic!("Unsupported image format"),
    };
    let mut texture_id: GLuint = 0;

    unsafe {
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut ebo);

        BindVertexArray(vao);

        // vertex buffer object
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            STATIC_DRAW,
        );

        // element buffer object
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            STATIC_DRAW,
        );

        // position attribute
        VertexAttribPointer(
            0,
            3,
            FLOAT,
            FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        EnableVertexAttribArray(0);

        // texture attribute
        VertexAttribPointer(
            1,
            2,
            FLOAT,
            FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        EnableVertexAttribArray(1);

        BindBuffer(ARRAY_BUFFER, 0);
        BindVertexArray(0);

        // Background texture
        GenTextures(1, &mut texture_id);
        BindTexture(TEXTURE_2D, texture_id);
        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

        TexImage2D(
            TEXTURE_2D,
            0,
            RGB as i32,
            image.width as i32,
            image.height as i32,
            0,
            RGB,
            UNSIGNED_BYTE,
            image.data.as_ptr() as *const _,
        );
        GenerateMipmap(TEXTURE_2D);
    }

    let shader = shader::Shader::new("src/shaders/shader.vert", "src/shaders/shader.frag");

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

            shader.use_program();
            let time = glfw.get_time() as f32;
            shader.set_uniform_float("millis", time);

            // Bind texture
            ActiveTexture(TEXTURE0);
            BindTexture(TEXTURE_2D, texture_id);
            shader.set_uniform_int("background", 0);

            BindVertexArray(vao);
            DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
