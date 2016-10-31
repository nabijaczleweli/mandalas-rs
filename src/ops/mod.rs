mod gen_ctx;

pub use self::gen_ctx::GenerationContext;

use std::fs::File;
use std::path::Path;
use image::{DynamicImage, ImageFormat};


/// Initialise the mandala canvas from the given size.
///
/// The resulting image is RGB.
///
/// # Examples
///
/// ```
/// # use mandalas::ops::init_image;
/// # struct O {
/// #     resolution: (usize, usize),
/// # }
/// # let opts = O { resolution: (900, 900) };
/// let mut mandala = init_image(opts.resolution);
/// // Process further...
/// ```
pub fn init_image(size: (usize, usize)) -> DynamicImage {
    DynamicImage::new_rgb8(size.0 as u32, size.1 as u32)
}

/// Check how many points need generating to acquire a smooth image for the given size.
///
/// This might not be 100% accurate but will try to be over- rather than underzealous.
///
/// # Examples
///
/// ```
/// # use mandalas::ops::points_to_generate;
/// # struct O {
/// #     resolution: (usize, usize),
/// # }
/// # let opts = O { resolution: (900, 900) };
/// for _ in 0..points_to_generate(opts.resolution) {
///     // Generate and set pixel
/// }
/// ```
pub fn points_to_generate(size: (usize, usize)) -> u64 {
    // (Closest power of 10 upwards) * 10
    10u64.pow(((size.0 * size.1) as f64).log10().ceil() as u32 + 1)
}

/// Generate a filename to save the mandala in.
pub fn filename_to_save(size: (usize, usize)) -> String {
    format!("mandala-{}x{}.png", size.0, size.1)
}

/// Save the mandala to the specified file
///
/// # Examples
///
/// ```
/// # use std::fs;
/// # use mandalas::Options;
/// # use std::env::temp_dir;
/// # use mandalas::ops::{filename_to_save, init_image, save};
/// # let opts = Options {
/// #     resolution: (900, 900),
/// #     outdir: ("$TEMP/mandalas-doctest/save-0".to_string(), temp_dir().join("mandalas-doctest").join("save-0")),
/// # };
/// # let _ = fs::create_dir_all(&opts.outdir.1);
/// # let mandala = init_image(opts.resolution);
/// save(&mandala, &opts.outdir.1, &filename_to_save(opts.resolution));
/// ```
pub fn save(image: &DynamicImage, outdir: &Path, fname: &str) {
    image.save(&mut File::create(outdir.join(fname)).unwrap(), ImageFormat::PNG).unwrap();
}
