extern crate mandalas;
extern crate image;
extern crate rand;
extern crate pbr;

use std::thread::Builder as ThreadBuilder;
use std::io::{Write, stdout};
use std::process::exit;
use std::path::PathBuf;
use image::RgbImage;
use std::sync::mpsc;


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
    progress.println(&format!("{}x{}x{} mandala from {} points",
                              opts.resolution.0,
                              opts.resolution.1,
                              opts.resolution.2,
                              mandalas::util::separated_number(pts)));

    let rxs = {
        let pts = pts / opts.threads_gen;
        let (txs, rxs): (Vec<_>, Vec<_>) = (0..opts.threads_coll).map(|_| mpsc::sync_channel(10)).unzip();

        progress.println(&format!("Generation threads: {}; granularity: {} points",
                                  opts.threads_gen,
                                  mandalas::util::separated_number(pts / 1000)));

        for gen_i in 0..opts.threads_gen {
            let mut pb = progress.create_bar(1000);
            pb.show_speed = false;
            pb.show_tick = false;

            let threads_coll = opts.threads_coll as usize;
            let txs = txs.clone();
            let res = opts.resolution;
            ThreadBuilder::new()
                .name(format!("generator {}", gen_i))
                .spawn(move || {
                    let mut gen = mandalas::ops::GenerationContext::new(res);

                    for _ in 0..1000 {
                        let mut pointss = vec![Vec::with_capacity(pts as usize / 1000); threads_coll];
                        for _ in 0..(pts / 1000) {
                            let point = gen.gen();
                            pointss[(point.0).2 as usize % threads_coll].push(point);
                        }

                        for (tx, points) in txs.iter().zip(pointss) {
                            tx.send(points).unwrap();
                        }
                        pb.inc();
                    }

                    pb.finish();
                })
                .unwrap();
        }

        rxs
    };

    progress.println("");
    progress.println(&format!("Collection threads: {}", opts.threads_coll));
    for (coll_i, rx) in rxs.into_iter().enumerate() {
        let mut pb = progress.create_bar(pts / opts.threads_coll);
        pb.show_speed = false;
        pb.show_tick = false;

        let threads_coll = opts.threads_coll as usize;
        let mut dummy_img = RgbImage::new(0, 0);
        let imgs: Vec<_> = imgs.iter_mut()
            .enumerate()
            .map(|(img_i, img)| {
                ImageContainer(if img_i % threads_coll == coll_i {
                    img.as_mut_rgb8().unwrap() as *mut _
                } else {
                    &mut dummy_img as *mut _
                })
            })
            .collect();
        ThreadBuilder::new()
            .name(format!("collector {}", coll_i))
            .spawn(move || {
                let mut imgs: Vec<_> = imgs.into_iter().map(|img| unsafe { &mut *img.0 }).collect();

                let mut acc = 0usize;
                for (i, points) in rx.iter().enumerate() {
                    let points_len = points.len();
                    for (pos, colour) in points {
                        imgs[pos.2 as usize][(pos.0, pos.1)].0 = colour;
                    }

                    if i % threads_coll == 0 {
                        pb.add(acc as u64);
                        acc = points_len;
                    } else {
                        acc += points_len;
                    }
                }
                pb.finish();
            })
            .unwrap();
    }

    progress.listen();

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
