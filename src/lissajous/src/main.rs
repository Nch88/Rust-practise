extern crate cairo;
extern crate rand;

use std::fs::File;

use cairo::{Context, Format, ImageSurface};

pub fn save_frame(surface: &ImageSurface, idx: u64) {
    let filename = format!("output/frame_{:0>4}.png", idx);
    let mut file = File::create(filename).expect("Couldn’t create file.");

    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");
}

pub fn lissajous_step(A: f64, a: f64, lambda: f64, B: f64, b: f64, t: u64) -> (f64, f64) {
    let x_sin: f64 = (a * t as f64 + lambda).sin();
    let x = A * x_sin;

    let y_sin: f64 = (b * t as f64).sin();
    let y: f64 = B * y_sin;

    (x, y)
}

pub fn draw_lissajous(surface: &ImageSurface, context: &Context) {
    let A = 30.0;
    let a = 3.0;
    let lambda = 5.0;
    let B = 40.0;
    let b = 4.0;

    let mut frames = 0;
    let mut stroke = || {
        context.stroke();
        save_frame(&surface, frames);
        frames += 1;
    };

    // draw 100 random black lines
    context.set_source_rgb(0.0, 0.0, 0.0);
    for t in 0..1000 {
        let (mut x, mut y) = lissajous_step(A, a, lambda, B, b, t);
        y += 100.0;
        x += 100.0;
        context.line_to(x, y);

        if t % 10 == 0 {
            stroke();
        }
    }
    stroke();
}

pub fn test_surface(surface: &ImageSurface, context: &Context) {
    let mut frames = 0;
    let mut stroke = || {
        context.stroke();
        save_frame(&surface, frames);
        frames += 1;
    };
    context.line_to(300.0, 300.0);
    // stroke();

    context.line_to(0.0, 300.0);
    stroke();
}

fn main() {
    let surface =
        ImageSurface::create(Format::ARgb32, 600, 600).expect("Failed to create a surface!");

    let context = Context::new(&surface);

    // paint canvas white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();

    // draw_lissajous(&surface, &context);
    test_surface(&surface, &context);
}
