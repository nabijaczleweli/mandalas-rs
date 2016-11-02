extern crate mandalas;
extern crate pbr;

use std::process::exit;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = mandalas::Options::parse();
    println!("{:#?}", opts);

    let mut img = mandalas::ops::init_image(opts.resolution);
    let mut gen = mandalas::ops::GenerationContext::new(opts.resolution);

    let pts = mandalas::ops::points_to_generate(opts.resolution);
    let mut pb = pbr::ProgressBar::new(pts / 1000);
    pb.show_speed = false;
    pb.show_tick = false;
    pb.message(&format!("A {}x{} mandala. Points [k]: ", opts.resolution.0, opts.resolution.1));
    for i in 0..pts {
        let ((x, y), colour) = gen.gen();
        img.as_mut_rgb8().unwrap().get_pixel_mut(x, y).data = colour;

        if i % 10000 == 0 {
            pb.add(10);
        }
    }
    pb.finish();

    mandalas::ops::save(&img, &opts.outdir.1, &mandalas::ops::filename_to_save(opts.resolution));

    Ok(())
}
