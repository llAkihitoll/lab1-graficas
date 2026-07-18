use raylib::prelude::*;

pub struct Framebuffer {
    pub image: Image,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        Framebuffer {
            image: Image::gen_image_color(width, height, background_color),
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.image.draw_pixel(x, y, color);
    }
}
