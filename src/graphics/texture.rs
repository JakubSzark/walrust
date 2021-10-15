#![allow(dead_code)]

use super::color::Color;
use image::{GenericImageView, ImageError};

pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Texture {
    /// Creates a blank black `Texture` of width and height
    pub fn new(width: usize, height: usize) -> Texture {
        Texture {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    /// Reads a image from file and creates a `Texture`
    pub fn from_image(path: &str) -> Result<Texture, ImageError> {
        let img = image::open(path)?;
        let mut texture = Texture::new(img.width() as usize, img.height() as usize);
        let buffer = img.as_rgba8().unwrap();

        for y in 0..texture.width {
            for x in 0..texture.height {
                let pixel = buffer.get_pixel(x as u32, y as u32);
                texture.pixels[y * texture.width + x] = Color {
                    red: pixel.0[0],
                    green: pixel.0[1],
                    blue: pixel.0[2],
                    alpha: pixel.0[3],
                };
            }
        }

        return Ok(texture);
    }

    /// Returns a pixel at the specified x and y coordinates
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Color::default()
        }
    }

    /// Sets a pixel at the specified x and y coordinates
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    /// Returns a pointer to the underlining pixel data
    pub unsafe fn as_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }

    /// Returns the width of the `Texture`
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the width of the `Texture`
    pub fn height(&self) -> usize {
        self.height
    }
}
