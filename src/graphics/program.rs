#![allow(dead_code)]

use super::shader::Shader;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new<const SIZE: usize>(shaders: [Shader; SIZE]) -> Result<Program, String> {
        unsafe {
            let id = gl::CreateProgram();
            for shader in shaders.iter() {
                gl::AttachShader(id, shader.id())
            }

            gl::LinkProgram(id);

            let mut status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let mut length = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut length);
                let mut log_buffer = vec![0u8; length as usize];
                gl::GetProgramInfoLog(id, length, &mut length, log_buffer.as_mut_ptr() as *mut i8);
                let log = String::from_utf8_unchecked(log_buffer);
                return Err(log);
            }

            for shader in shaders {
                shader.invalidate();
            }

            Ok(Program { id })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
