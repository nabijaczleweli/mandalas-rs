extern crate mandalas;
extern crate pbr;

use std::io::{Write, stdout};
use std::process::exit;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = mandalas::Options::parse();
    println!("{:#?}", opts);

    print!("Allocating {}kB image...", mandalas::util::separated_number((opts.resolution.0 * opts.resolution.1 * 3 / 1024) as u64));
    stdout().flush().unwrap();
    let mut img = mandalas::ops::init_image(opts.resolution);
    println!(" Done");

    let pts = mandalas::ops::points_to_generate(opts.resolution);

    let rx = {
        let pts = pts / opts.threads;
        let (tx, rx) = mpsc::channel();

        for _ in 0..opts.threads {
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
    pb.message(&format!("A {}x{} mandala on {} thread{}. Points [k]: ",
                        opts.resolution.0,
                        opts.resolution.1,
                        opts.threads,
                        if opts.threads == 1 { "" } else { "s" }));
    {
        let ref mut img = img.as_mut_rgb8().unwrap();
        for (i, ((x, y), colour)) in rx.iter().enumerate() {
            img.get_pixel_mut(x, y).data = colour;

            if i % 25000 == 0 {
                pb.add(25);
            }
        }
    }
    pb.finish_print("");

    let fname = mandalas::ops::filename_to_save(opts.resolution);
    print!("Saving to {}...", PathBuf::from(&opts.outdir.0).join(&fname).to_str().unwrap().replace('\\', "/"));
    stdout().flush().unwrap();
    mandalas::ops::save(&img, &opts.outdir.1, &fname);
    println!(" Done");

    Ok(())
}
