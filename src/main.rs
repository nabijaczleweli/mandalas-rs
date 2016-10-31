extern crate arraydeque;
extern crate mandalas;
extern crate image;

use std::process::exit;
use std::fs::File;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = mandalas::Options::parse();
    println!("{:#?}", opts);

    let mut img = mandalas::ops::init_image(opts.resolution);
    let mut gen = mandalas::ops::GenerationContext::new(opts.resolution);

    println!("10^{} points", (mandalas::ops::points_to_generate(opts.resolution) as f64).log10());
    for _ in 0..mandalas::ops::points_to_generate(opts.resolution) {
        let ((x, y), colour) = gen.gen();
        img.as_mut_rgb8().unwrap().get_pixel_mut(x, y).data = colour;
    }

    img.save(&mut File::create("mandala.png").unwrap(), image::ImageFormat::PNG).unwrap();

    Ok(())
}
