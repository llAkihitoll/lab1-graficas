use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

// revisa si un punto esta dentro de un poligono (regla par-impar)
// se usa para saber si un pixel cae dentro del agujero
fn punto_en_poligono(px: i32, py: i32, puntos: &[(i32, i32)]) -> bool {
    let n = puntos.len();
    let mut dentro = false;
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = puntos[i];
        let (xj, yj) = puntos[j];

        if (yi > py) != (yj > py) {
            let x_corte = xi as f32 + (py - yi) as f32 * (xj - xi) as f32 / (yj - yi) as f32;
            if (px as f32) < x_corte {
                dentro = !dentro;
            }
        }
        j = i;
    }

    dentro
}

// relleno por scanline: por cada fila Y calcula donde la fila cruza
// las aristas del poligono y pinta entre cada par de cruces
// si se pasa un agujero, los pixeles dentro de ese agujero no se pintan
pub fn rellenar_poligono(
    fb: &mut Framebuffer,
    puntos: &[(i32, i32)],
    color: Color,
    agujero: Option<&[(i32, i32)]>,
) {
    let n = puntos.len();
    let y_min = puntos.iter().map(|p| p.1).min().unwrap();
    let y_max = puntos.iter().map(|p| p.1).max().unwrap();

    for y in y_min..=y_max {
        let mut cruces: Vec<i32> = Vec::new();
        let mut j = n - 1;

        for i in 0..n {
            let (xi, yi) = puntos[i];
            let (xj, yj) = puntos[j];

            if (yi > y) != (yj > y) {
                let x_corte = xi as f32 + (y - yi) as f32 * (xj - xi) as f32 / (yj - yi) as f32;
                cruces.push(x_corte.round() as i32);
            }
            j = i;
        }

        cruces.sort();

        let mut k = 0;
        while k + 1 < cruces.len() {
            let x_ini = cruces[k];
            let x_fin = cruces[k + 1];

            for x in x_ini..=x_fin {
                if let Some(hoyo) = agujero {
                    if punto_en_poligono(x, y, hoyo) {
                        continue;
                    }
                }
                fb.set_pixel(x, y, color);
            }
            k += 2;
        }
    }
}
