//! OpenGL ES Shader utilities

use gl;
use std::ffi::CString;
use std::ptr;

/// Shader program wrapper
pub struct ShaderProgram {
    pub id: u32,
}

impl ShaderProgram {
    /// Create a new shader program from vertex and fragment shaders
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        let vertex_shader = Self::compile_shader(vertex_src, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(fragment_src, gl::FRAGMENT_SHADER)?;

        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // Check link status
            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            if status == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut log = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, &mut len, log.as_mut_ptr() as *mut _);
                let log_str = String::from_utf8_lossy(&log);
                return Err(format!("Shader link error: {}", log_str));
            }
        }

        // Delete shaders (they're now attached to program)
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(Self { id: program })
    }

    /// Compile a single shader
    fn compile_shader(source: &str, shader_type: u32) -> Result<u32, String> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_source = CString::new(source).map_err(|e| e.to_string())?;

        unsafe {
            gl::ShaderSource(
                shader,
                1,
                &c_source.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(shader);

            // Check compile status
            let mut status = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut log = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, &mut len, log.as_mut_ptr() as *mut _);
                let log_str = String::from_utf8_lossy(&log);
                return Err(format!("Shader compile error: {}", log_str));
            }
        }

        Ok(shader)
    }

    /// Use the shader program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Get uniform location
    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        let c_name = CString::new(name).ok()?;
        let location = unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) };
        if location == -1 {
            None
        } else {
            Some(location)
        }
    }

    /// Set uniform mat4
    pub fn set_uniform_mat4(&self, location: i32, matrix: &[f32; 16]) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }

    /// Set uniform vec3
    pub fn set_uniform_vec3(&self, location: i32, vector: &[f32; 3]) {
        unsafe {
            gl::Uniform3fv(location, 1, vector.as_ptr());
        }
    }

    /// Set uniform float
    pub fn set_uniform_float(&self, location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
