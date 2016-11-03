extern crate mandalas;
extern crate pbr;

use std::process::exit;
use std::sync::mpsc;
use std::thread;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = mandalas::Options::parse();
    println!("{:#?}", opts);

    let mut img = mandalas::ops::init_image(opts.resolution);
    let pts = mandalas::ops::points_to_generate(opts.resolution);

    let rx = {
        let pts = pts / 8;
        let (tx, rx) = mpsc::channel();

        for _ in 0..8 {
            let tx = tx.clone();
            let res = opts.resolution;
            thread::spawn(move || {
                let mut gen = mandalas::ops::GenerationContext::new(res);

                for _ in 0..pts {
                    tx.send(gen.gen()).unwrap();
                }
            });
        }
        rx
    };

    let mut pb = pbr::ProgressBar::new(pts / 1000);
    pb.show_speed = false;
    pb.show_tick = false;
    pb.message(&format!("A {}x{} mandala on 8 threads. Points [k]: ", opts.resolution.0, opts.resolution.1));
    {
        let ref mut img = img.as_mut_rgb8().unwrap();
        for (i, ((x, y), colour)) in rx.iter().enumerate() {
            img.get_pixel_mut(x, y).data = colour;

            if i % 10000 == 0 {
                pb.add(10);
            }
        }
    }
    pb.finish();

    mandalas::ops::save(&img, &opts.outdir.1, &mandalas::ops::filename_to_save(opts.resolution));

    Ok(())
}
