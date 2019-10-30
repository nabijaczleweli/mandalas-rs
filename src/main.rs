extern crate mandalas;
extern crate image;
extern crate rand;
extern crate pbr;

use std::io::{Write, stdout};
use std::process::exit;
use std::path::PathBuf;
use image::RgbImage;
use std::sync::mpsc;
use std::thread;


struct ImageContainer(*mut RgbImage);
unsafe impl Send for ImageContainer {}


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
    println!();

    let pts = mandalas::ops::points_to_generate(opts.resolution);

    let mut progress = pbr::MultiBar::new();
    progress.println(&format!("{}x{}x{} mandala", opts.resolution.0, opts.resolution.1, opts.resolution.2));

    let rx = {
        let pts = pts / opts.threads;
        let (tx, rx) = mpsc::sync_channel(10);

        progress.println(&format!("Generation threads: {}; granularity: {} points",
                                  opts.threads,
                                  mandalas::util::separated_number(pts / 100)));

        for _ in 0..opts.threads {
            let mut pb = progress.create_bar(100);
            pb.show_speed = false;
            pb.show_tick = false;

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
                    pb.inc();
                }

                pb.finish();
            });
        }

        rx
    };

    progress.println("");
    progress.println(&format!("Collection threads: 1; granularity: {} points", mandalas::util::separated_number(1000000)));
    {
        let mut pb = progress.create_bar(pts / 1000000);
        pb.show_speed = false;
        pb.show_tick = false;

        let mut imgs: Vec<_> = imgs.iter_mut().map(|img| ImageContainer(img.as_mut_rgb8().unwrap() as *mut _)).collect();
        thread::spawn(move || {
            let mut imgs: Vec<_> = imgs.drain(..).map(|img| unsafe { &mut *img.0 }).collect();

            for (i, (pos, colour)) in rx.iter().flatten().enumerate() {
                imgs[pos.2 as usize][(pos.0, pos.1)].0 = colour;

                if i % (15 * 1000000) == 0 {
                    pb.add(15);
                }
            }
            pb.finish();
        });
    }

    progress.listen();

    // let imgs = imgs.join().expect("Collection thread panicked");
    for z in 0..opts.resolution.2 {
        let fname = mandalas::ops::filename_to_save(opts.resolution, z);
        print!("Saving to {}...",
               PathBuf::from(&opts.outdir.0).join(&fname).to_str().unwrap().replace('\\', "/"));
        stdout().flush().unwrap();
        mandalas::ops::save(&imgs[z], &opts.outdir.1, &fname);
        println!(" Done");
    }

    Ok(())
}
