extern crate mandalas;
extern crate image;
extern crate rand;
extern crate pbr;

use std::thread::Builder as ThreadBuilder;
use image::{DynamicImage, RgbImage};
use std::io::{Write, stdout};
use std::process::exit;
use std::sync::mpsc;
use std::fs;


/// In how many chunks to generate-send the a generator thread's points
///
/// Each generator thread generates and sends `pts / threads_gen` points,
/// but groups them into `GENERATION_CHUNKS` discrete chunks for processing.
const GENERATION_CHUNKS: u64 = 1000;

/// How many generation chunks to cache per layer group
///
/// Note that high (on the order of 100, 1000) values will drastically increase worst-case (collection stalled) memory usage
///
/// This roughly works out to
/// ```
/// QUEUE_DEPTH * (pts / threads_gen / GENERATION_CHUNKS) * threads_coll * (sizeof(gen()) == 8)
/// ```
///
/// So for a queue depth of 10, 1000 chunks, a 2160x image => 10^12 points on 19/5 threads
/// this works out to up to 21 gigabytes of additional memory.
const QUEUE_DEPTH: usize = 10;


struct RgbImageContainer(*mut RgbImage);
unsafe impl Send for RgbImageContainer {}

struct DynamicImagesContainer(*const Vec<DynamicImage>);
unsafe impl Send for DynamicImagesContainer {}


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
        let (txs, rxs): (Vec<_>, Vec<_>) = (0..opts.threads_coll).map(|_| mpsc::sync_channel(QUEUE_DEPTH)).unzip();

        progress.println(&format!("Generation threads: {}; granularity: {} points",
                                  opts.threads_gen,
                                  mandalas::util::separated_number(pts / GENERATION_CHUNKS)));

        for gen_i in 0..opts.threads_gen {
            let mut pb = progress.create_bar(GENERATION_CHUNKS);
            pb.show_speed = false;
            pb.show_tick = false;

            let threads_coll = opts.threads_coll as usize;
            let txs = txs.clone();
            let res = opts.resolution;
            let thread = ThreadBuilder::new()
                .name(format!("generator {}", gen_i))
                .spawn(move || {
                    let mut gen = mandalas::ops::GenerationContext::new(res);

                    for _ in 0..GENERATION_CHUNKS {
                        let mut pointss = vec![Vec::with_capacity((pts / GENERATION_CHUNKS) as usize); threads_coll];
                        for _ in 0..(pts / GENERATION_CHUNKS) {
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
            if opts.affine_threads {
                mandalas::ops::affine_thread(&thread, gen_i as usize);
            }
        }

        rxs
    };

    progress.println("");
    progress.println(&format!("Collection threads: {}", opts.threads_coll));
    let mut dummy_img = RgbImage::new(opts.resolution.0 as u32, opts.resolution.1 as u32);
    for (coll_i, rx) in rxs.into_iter().enumerate() {
        let mut pb = progress.create_bar(pts / opts.threads_coll);
        pb.show_speed = false;
        pb.show_tick = false;

        let threads_coll = opts.threads_coll as usize;
        let imgs: Vec<_> = imgs.iter_mut()
            .enumerate()
            .map(|(img_i, img)| {
                RgbImageContainer(if img_i % threads_coll == coll_i {
                    img.as_mut_rgb8().unwrap() as *mut _
                } else {
                    &mut dummy_img as *mut _
                })
            })
            .collect();
        let thread = ThreadBuilder::new()
            .name(format!("collector {}", coll_i))
            .spawn(move || {
                // This unsafe is sound because we only use layers whose number mod threads_coll matchers our coll_i,
                // and both top-level imgs and dummy_img live past this thread which terminates before progress.listen() returns below,
                // so while there exist hundreds of mutable references to dummy_img, none of them are actually used (hopefully).
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
        if opts.affine_threads {
            mandalas::ops::affine_thread(&thread, opts.threads_gen as usize + coll_i);
        }
    }

    progress.listen();
    if dummy_img.into_raw().into_iter().find(|&c| c != 0).is_some() {
        eprintln!("warning: dummy image not empty");
    }
    println!();


    let threads_save = opts.threads_gen + opts.threads_coll;
    let mut progress = pbr::MultiBar::new();
    progress.println(&format!("Saving {} images on {} threads", opts.resolution.2, threads_save));

    // This should be ensured at option validation stage, but shit happens,
    // and zombieing forever when one the threads panic disowned with 30 gigs of ram is less than desirable
    fs::create_dir_all(&opts.outdir.1).unwrap();
    for save_i in 0..threads_save {
        let mut pb = progress.create_bar(opts.resolution.2 as u64 / threads_save);
        pb.show_speed = false;
        pb.show_tick = false;

        let resolution = opts.resolution;
        let outdir = opts.outdir.1.clone();
        let imgs = DynamicImagesContainer(&imgs as *const _);
        ThreadBuilder::new()
            .name(format!("saver {}", save_i))
            .spawn(move || {
                // This unsafe is sound because (a) this thread exits before progress.listen() call below returns,
                // abd top-level imgs lives longer than that and (b) this is an immutable reference.
                let imgs = unsafe { &*imgs.0 as &Vec<DynamicImage> };

                for z in (save_i as usize..resolution.2).step_by(threads_save as usize) {
                    imgs[z].save(outdir.join(mandalas::ops::filename_to_save(resolution, z))).unwrap();
                    pb.inc();
                }

                pb.finish();
            })
            .unwrap();
    }

    progress.listen();

    Ok(())
}
