# Lab 1 - Relleno de poligonos

Laboratorio de Graficas por Computadora. El objetivo es dibujar varios
poligonos (algunos concavos, de mas de 4 vertices) usando un framebuffer
propio, respetando sus bordes y colores de relleno. Uno de los poligonos
representa un agujero dentro de otro y no debe rellenarse.

## Estructura del proyecto

```
src/
  framebuffer.rs   struct Framebuffer, wrapper sobre raylib::Image
  line.rs          algoritmos de linea (ecuacion, DDA, Bresenham)
  fill.rs          relleno de poligonos por scanline
  main.rs          datos de los poligonos del lab y render de la escena
```

## Algoritmo de relleno

`fill.rs` implementa scanline con regla par-impar: por cada fila Y del
poligono se calculan las intersecciones con todas las aristas, se ordenan
en X y se pintan los pixeles entre cada par de cruces. Esto funciona bien
con poligonos concavos sin necesitar flood fill.

Para el agujero (poligono 5 dentro del poligono 4), `rellenar_poligono`
recibe opcionalmente los vertices del agujero: antes de pintar cada pixel
se revisa si cae dentro de esos vertices (misma regla par-impar) y si es
asi se salta. El agujero solo se dibuja con su borde, nunca se rellena.

Los bordes de todos los poligonos se dibujan con `draw_line` (Bresenham,
ya existente en `line.rs`) despues del relleno, para que el borde no quede
tapado por el color de relleno.

## Requisitos

- Rust + Cargo (edicion 2021 o superior).
- Un compilador de C y CMake instalados y accesibles desde la terminal
  (raylib-sys compila raylib desde codigo fuente en el primer build). En
  Windows, la forma mas simple es tener MSYS2/MinGW-w64 con `gcc` y
  `cmake` en el PATH.

## Como ejecutar

```
cargo run
```

Esto compila el proyecto (la primera vez tarda mas porque compila raylib)
y ejecuta `main.rs`, que arma el framebuffer, dibuja y rellena los 5
poligonos, y al final llama a `fb.image.export_image("out.png")`. El
archivo `out.png` queda en la raiz del proyecto.

## Pruebas sugeridas

- Revisar `out.png` a ojo: los 4 poligonos rellenos deben verse solidos,
  sin huecos entre filas ni pixeles fuera del contorno.
- Confirmar que el agujero (dentro del poligono 4) se ve del color de
  fondo, con su borde negro, y no del color de relleno del poligono 4.
- Cambiar el orden de los vertices de un poligono (sentido horario vs
  antihorario) y verificar que el relleno sigue siendo correcto, ya que
  el algoritmo no depende de la orientacion.
- Probar con un poligono nuevo de mas de 4 vertices para confirmar que
  `rellenar_poligono` no esta hardcodeado a una cantidad fija de puntos.
