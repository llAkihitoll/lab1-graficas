use raylib::prelude::*;

mod framebuffer;
mod line;

use framebuffer::Framebuffer;

fn main() {
    let mut fb = Framebuffer::new(800, 450, Color::WHITE);

    fb.image.export_image("out.png");
}
