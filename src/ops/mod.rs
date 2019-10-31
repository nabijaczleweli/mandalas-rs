//! Main functions doing actual work.
//!
//! # The Tonči Juričev-Grgin mandala generation method
//!
//! First, choose a corner.


mod gen_ctx;

pub use self::gen_ctx::GenerationContext;

use image::DynamicImage;


/// Initialise the mandala canvas from the given size.
///
/// The resulting image is RGB.
///
/// # Examples
///
/// ```
/// # use mandalas::ops::init_image;
/// # struct O {
/// #     resolution: (usize, usize, usize),
/// # }
/// # let opts = O { resolution: (900, 900, 0) };
/// let mut mandala = init_image(opts.resolution);
/// // Process further...
/// ```
pub fn init_image(size: (usize, usize, usize)) -> Vec<DynamicImage> {
    (0..size.2).map(|_| DynamicImage::new_rgb8(size.0 as u32, size.1 as u32)).collect()
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
/// #     resolution: (usize, usize, usize),
/// # }
/// # let opts = O { resolution: (900, 900, 0) };
/// for _ in 0..points_to_generate(opts.resolution) {
///     // Generate and set pixel
/// }
/// ```
pub fn points_to_generate(size: (usize, usize, usize)) -> u64 {
    // (Closest power of 10 upwards) * 10
    10u64.pow(((size.0 * size.1 * size.2) as f64).log10().ceil() as u32 + 1)
}

/// Generate a filename to save the mandala in.
pub fn filename_to_save(size: (usize, usize, usize), z: usize) -> String {
    format!("mandala-{}x{}x{}-{:05}.png", size.0, size.1, size.2, z)
}
