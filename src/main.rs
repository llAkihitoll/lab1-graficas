use raylib::prelude::*;

mod fill;
mod framebuffer;
mod line;

use fill::rellenar_poligono;
use framebuffer::Framebuffer;
use line::draw_line;

// dibuja el contorno de un poligono uniendo cada vertice con el siguiente
// y cerrando del ultimo vertice al primero
fn dibujar_borde(fb: &mut Framebuffer, puntos: &[(i32, i32)], color: Color) {
    let n = puntos.len();
    for i in 0..n {
        let (x0, y0) = puntos[i];
        let (x1, y1) = puntos[(i + 1) % n];
        draw_line(fb, x0, y0, x1, y1, color);
    }
}

fn main() {
    let mut fb = Framebuffer::new(800, 450, Color::WHITE);

    let poligono1 = [
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];

    let poligono2 = [(321, 335), (288, 286), (339, 251), (374, 302)];

    let poligono3 = [(377, 249), (411, 197), (436, 249)];

    let poligono4 = [
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ];

    // agujero dentro del poligono 4, solo se dibuja el borde
    let agujero = [(682, 175), (708, 120), (735, 148), (739, 170)];

    let borde = Color::BLACK;

    rellenar_poligono(&mut fb, &poligono1, Color::RED, None);
    dibujar_borde(&mut fb, &poligono1, borde);

    rellenar_poligono(&mut fb, &poligono2, Color::GREEN, None);
    dibujar_borde(&mut fb, &poligono2, borde);

    rellenar_poligono(&mut fb, &poligono3, Color::BLUE, None);
    dibujar_borde(&mut fb, &poligono3, borde);

    rellenar_poligono(&mut fb, &poligono4, Color::YELLOW, Some(&agujero));
    dibujar_borde(&mut fb, &poligono4, borde);

    dibujar_borde(&mut fb, &agujero, borde);

    fb.image.export_image("out.png");
}
