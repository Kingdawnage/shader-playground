#![allow(dead_code)]
use gl::*;
use std::{ffi::CString, fs::File, io::Read, ptr, str};

pub struct Shader {
    vert: String,
    frag: String,
    pub program_id: u32,
}

impl Shader {
    pub fn new(vert: &str, frag: &str) -> Shader {
        let mut shader = Shader {
            vert: vert.to_string(),
            frag: frag.to_string(),
            program_id: 0,
        };
        shader.compile_shader();
        shader
    }

    pub fn compile_shader(&mut self) {
        unsafe {
            let vert_shader = CreateShader(VERTEX_SHADER);
            let frag_shader = CreateShader(FRAGMENT_SHADER);

            // Read vertex shader source
            let mut file = File::open(&self.vert).expect("Failed to open file");
            let mut vert_shader_src = String::new();
            file.read_to_string(&mut vert_shader_src)
                .expect("Failed to read file");
            let cstr_vert = CString::new(vert_shader_src.as_bytes()).unwrap();

            // Read fragment shader source
            let mut file = File::open(&self.frag).expect("Failed to open file");
            let mut frag_shader_src = String::new();
            file.read_to_string(&mut frag_shader_src)
                .expect("Failed to read file");
            let cstr_frag = CString::new(frag_shader_src.as_bytes()).unwrap();

            // Compile vertex shader
            ShaderSource(vert_shader, 1, &cstr_vert.as_ptr(), ptr::null());
            CompileShader(vert_shader);

            // Check for vertex shader compile errors
            let mut success = 0;
            let mut info_log: Vec<i8> = Vec::with_capacity(512);
            GetShaderiv(vert_shader, COMPILE_STATUS, &mut success);
            if success == 0 {
                GetShaderInfoLog(vert_shader, 512, ptr::null_mut(), info_log.as_mut_ptr());
                let info_log_u8: Vec<u8> = info_log.iter().map(|&c| c as u8).collect();
                println!(
                    "Error: Vertex shader compilation failed: {}",
                    str::from_utf8(&info_log_u8).unwrap()
                );
            }

            // Compile fragment shader
            ShaderSource(frag_shader, 1, &cstr_frag.as_ptr(), ptr::null());
            CompileShader(frag_shader);

            // Check for fragment shader compile errors
            GetShaderiv(frag_shader, COMPILE_STATUS, &mut success);
            if success == 0 {
                GetShaderInfoLog(frag_shader, 512, ptr::null_mut(), info_log.as_mut_ptr());
                let info_log_u8: Vec<u8> = info_log.iter().map(|&c| c as u8).collect();
                println!(
                    "Error: Fragment shader compilation failed: {}",
                    str::from_utf8(&info_log_u8).unwrap()
                );
            }

            // Create shader program and link the shaders
            self.program_id = CreateProgram();
            AttachShader(self.program_id, vert_shader);
            AttachShader(self.program_id, frag_shader);
            LinkProgram(self.program_id);

            // Check for linking errors
            GetProgramiv(self.program_id, LINK_STATUS, &mut success);
            if success == 0 {
                GetProgramInfoLog(self.program_id, 512, ptr::null_mut(), info_log.as_mut_ptr());
                let info_log_u8: Vec<u8> = info_log.iter().map(|&c| c as u8).collect();
                println!(
                    "Error: Shader program linking failed: {}",
                    str::from_utf8(&info_log_u8).unwrap()
                );
            }

            // Delete shaders
            DeleteShader(vert_shader);
            DeleteShader(frag_shader);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            UseProgram(self.program_id);
        }
    }

    pub fn set_uniform_2fv(&self, name: &str, value: Vec<f32>) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform2fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_3fv(&self, name: &str, value: Vec<f32>) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform3fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_4fv(&self, name: &str, value: Vec<f32>) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform4fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform1f(location, value);
        }
    }

    pub fn set_uniform_int(&self, name: &str, value: i32) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform1i(location, value);
        }
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        let location = unsafe {
            let uniform_name = CString::new(name).unwrap();
            GetUniformLocation(self.program_id, uniform_name.as_ptr())
        };

        unsafe {
            Uniform1i(location, value as i32);
        }
    }
}
