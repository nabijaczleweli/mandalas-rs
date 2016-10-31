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
