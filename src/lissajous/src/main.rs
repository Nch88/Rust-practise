extern crate cairo;

use cairo::{ImageSurface, Format, Context};

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600);

}
