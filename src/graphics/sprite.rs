#![allow(dead_code)]

use super::texture::Texture;

pub struct Sprite {
    texture: Texture,
    id: u32,
}

impl Sprite {
    pub fn from_texture(texture: Texture) -> Sprite {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                texture.width() as i32,
                texture.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture.as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            Sprite { texture, id }
        }
    }

    pub fn refresh(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                self.texture.width() as i32,
                self.texture.height() as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                self.texture.as_ptr() as *const _,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
