#![allow(dead_code)]

pub struct Shader {
    id: u32,
}

pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
}

impl Shader {
    pub fn from_source(source: &str, stype: ShaderType) -> Result<Shader, String> {
        unsafe {
            let id = gl::CreateShader(match stype {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
                ShaderType::Geometry => gl::GEOMETRY_SHADER,
            });

            let source_ptr = source.as_ptr() as *const i8;
            let source_len = source.len() as i32;

            gl::ShaderSource(id, 1, &source_ptr, &source_len);
            gl::CompileShader(id);

            let mut status = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

            if status == 0 {
                let mut length = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
                let mut log_buffer = vec![0u8; length as usize];
                gl::GetShaderInfoLog(id, length, &mut length, log_buffer.as_mut_ptr() as *mut i8);
                let log = String::from_utf8_unchecked(log_buffer);
                return Err(log);
            }

            Ok(Shader { id })
        }
    }

    pub fn invalidate(self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
