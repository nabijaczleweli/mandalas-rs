extern crate mandalas;
extern crate rand;
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

    print!("Allocating {}kB of images...",
           mandalas::util::separated_number((opts.resolution.0 * opts.resolution.1 * opts.resolution.2 * 3 / 1024) as u64));
    stdout().flush().unwrap();
    let mut imgs = mandalas::ops::init_image(opts.resolution);
    println!(" Done");

    let pts = mandalas::ops::points_to_generate(opts.resolution);

    let rx = {
        let pts = pts / opts.threads;
        let (tx, rx) = mpsc::sync_channel(100);

        for _ in 0..opts.threads {
            let tx = tx.clone();
            let res = opts.resolution;
            thread::spawn(move || {
                let mut gen = mandalas::ops::GenerationContext::new(res);

                for _ in 0..100 {
                    let mut points = vec![gen.gen(); pts as usize / 100];
                    for p in &mut points[1..] {
                        *p = gen.gen();
                    }
                    tx.send(points).unwrap();
                }
            });
        }

        rx
    };

    let mut pb = pbr::ProgressBar::new(pts / 1000000);
    pb.show_speed = false;
    pb.show_tick = false;
    pb.message(&format!("A {}x{}x{} mandala on {} thread{}. Points [M]: ",
                        opts.resolution.0,
                        opts.resolution.1,
                        opts.resolution.2,
                        opts.threads,
                        if opts.threads == 1 { "" } else { "s" }));
    {
        let ref mut imgs: Vec<_> = imgs.iter_mut().map(|img| img.as_mut_rgb8().unwrap()).collect();
        for (i, (pos, colour)) in rx.iter().flatten().enumerate() {
            imgs[pos.2 as usize][(pos.0, pos.1)].data = colour;

            if i % 85000000 == 0 {
                pb.add(85);
            }
        }
    }
    pb.finish_print("");

    for y in 0..opts.resolution.2 {
        let fname = mandalas::ops::filename_to_save(opts.resolution, y);
        print!("Saving to {}...",
               PathBuf::from(&opts.outdir.0).join(&fname).to_str().unwrap().replace('\\', "/"));
        stdout().flush().unwrap();
        mandalas::ops::save(&imgs[y], &opts.outdir.1, &fname);
        println!(" Done");
    }

    Ok(())
}
