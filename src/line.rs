use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub fn linea_ecuacion(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let m = (y1 - y0) as f32 / (x1 - x0) as f32;
    let b = y0 as f32 - m * x0 as f32;

    for x in x0..=x1 {
        let y = m * x as f32 + b;
        fb.set_pixel(x, y.round() as i32, color);
    }
}

pub fn dda(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let pasos = dx.abs().max(dy.abs());

    let x_inc = dx as f32 / pasos as f32;
    let y_inc = dy as f32 / pasos as f32;

    let mut x = x0 as f32;
    let mut y = y0 as f32;

    for _ in 0..pasos {
        fb.set_pixel(x.round() as i32, y.round() as i32, color);
        x += x_inc;
        y += y_inc;
    }
}

// Bresenham
pub fn draw_line(fb: &mut Framebuffer, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.set_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
