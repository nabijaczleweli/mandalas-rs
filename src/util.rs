//! Various utility functions.


use itertools::Itertools;


/// Calculate the distance between two specified 3-dimensional points.
pub fn distance(from: (usize, usize, usize), to: (usize, usize, usize)) -> f64 {
    let x = (from.0 as isize) - (to.0 as isize);
    let y = (from.1 as isize) - (to.1 as isize);
    let z = (from.2 as isize) - (to.2 as isize);
    ((x * x + y * y + z * z) as f64).sqrt()
}

/// Convert the specified number to a string in the form "1'234'567"
///
/// # Examples
///
/// ```
/// # use mandalas::util::separated_number;
/// assert_eq!(separated_number(1000), "1'000".to_string());
/// assert_eq!(separated_number(100000), "100'000".to_string());
/// ```
pub fn separated_number(num: u64) -> String {
    num.to_string()
        .chars()
        .rev()
        .chunks(3)
        .into_iter()
        .fold(String::new(), |mut s, c| {
            s.extend(c);
            s + "'"
        })
        .chars()
        .rev()
        .skip(1)
        .collect()
}

/// Translate a CMYK colour point into an RGB colour point.
///
/// # Examples
///
/// ```
/// # use mandalas::util::cmyk_to_rgb;
/// assert_eq!(cmyk_to_rgb([0f64, 0f64, 0f64, 0f64]), [1f64, 1f64, 1f64]);
/// assert_eq!(cmyk_to_rgb([0f64, 1f64, 0f64, 0f64]), [1f64, 0f64, 1f64]);
/// ```
pub fn cmyk_to_rgb(cmyk: [f64; 4]) -> [f64; 3] {
    [(1f64 - cmyk[0]) * (1f64 - cmyk[3]), (1f64 - cmyk[1]) * (1f64 - cmyk[3]), (1f64 - cmyk[2]) * (1f64 - cmyk[3])]
}

/// Translate an HSL colour point into an RGB colour point.
///
/// Based wholly on https://en.wikipedia.org/wiki/HSL_and_HSV#From_HSL.
///
/// # Examples
///
/// ```
/// # use mandalas::util::hsl_to_rgb;
/// assert_eq!(hsl_to_rgb([1f64, 1f64, 1f64]), [1f64, 1f64, 1f64]);
/// assert_eq!(hsl_to_rgb([0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn hsl_to_rgb(hsl: [f64; 3]) -> [f64; 3] {
    let h_deg = hsl[0] * 360f64;
    let h_prim = h_deg / 60f64;

    let chroma = (1f64 - (2f64 * hsl[2] - 1f64)) * hsl[1];
    let x = chroma * (1f64 - ((h_prim % 2f64) - 1f64).abs());

    let rgb1 = if 0f64 <= h_prim && h_prim <= 1f64 {
        [chroma, x, 0f64]
    } else if 1f64 <= h_prim && h_prim <= 2f64 {
        [x, chroma, 0f64]
    } else if 2f64 <= h_prim && h_prim <= 3f64 {
        [0f64, chroma, x]
    } else if 3f64 <= h_prim && h_prim <= 4f64 {
        [0f64, x, chroma]
    } else if 4f64 <= h_prim && h_prim <= 5f64 {
        [x, 0f64, chroma]
    } else if 5f64 <= h_prim && h_prim < 6f64 {
        [chroma, 0f64, x]
    } else {
        [0f64, 0f64, 0f64]
    };

    let m = hsl[2] - (chroma / 2f64);
    [rgb1[0] + m, rgb1[1] + m, rgb1[2] + m]
}

/// Translate an RGB colour point into a Sepia tone.
///
/// Based on https://stackoverflow.com/a/9449159/2851815.
///
/// # Examples
///
/// ```
/// # use mandalas::util::rgb_to_sepia;
/// assert_eq!(rgb_to_sepia([0.5, 0.5, 0.5]), [0.6755, 0.6015, 0.4685]);
/// assert_eq!(rgb_to_sepia([0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn rgb_to_sepia(rgb: [f64; 3]) -> [f64; 3] {
    [((rgb[0] * 0.393) + (rgb[1] * 0.769) + (rgb[2] * 0.189)).min(1.),
     ((rgb[0] * 0.349) + (rgb[1] * 0.686) + (rgb[2] * 0.168)).min(1.),
     ((rgb[0] * 0.272) + (rgb[1] * 0.534) + (rgb[2] * 0.131)).min(1.)]
}

/// Translate a YUV colour point (translated to 0–1 as opposed to -0.5–0.5)
/// into an RGB colour point according to the BT.709 recommendation.
///
/// Based on https://en.wikipedia.org/wiki/YUV#HDTV_with_BT.709, https://goo.gl/kd6DPm and https://goo.gl/C9w4qT (original URLs too long).
///
/// Effectively computes
///
/// ![YUV to RGB matrix
/// multiplicaiton](https://wikimedia.org/api/rest_v1/media/math/render/svg/9d783bf1ed1edbda3e8e87d3d9067bd156a5e75b)
///
/// # Examples
///
/// ```
/// # use mandalas::util::yuv_to_rgb;
/// assert_eq!(yuv_to_rgb([0f64, 0f64, 0f64]), [0f64, 0.297705, 0f64]);
/// ```
pub fn yuv_to_rgb(yuv: [f64; 3]) -> [f64; 3] {
    // Scaled to [0-1, -0.5-0.5, -0.5-0.5]
    let yuv = [yuv[0], yuv[1] - 0.5, yuv[2] - 0.5];
    [(1.28033 * yuv[2] + yuv[0]).max(0f64).min(1f64),
     (-0.21482 * yuv[1] - 0.38059 * yuv[2] + yuv[0]).max(0f64).min(1f64),
     (2.12798 * yuv[1] + yuv[0]).max(0f64).min(1f64)]
}
