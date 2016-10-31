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
