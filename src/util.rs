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
/// Based on https://en.wikipedia.org/wiki/YUV#HDTV_with_BT.709, https://goo.gl/kd6DPm and https://goo.gl/C9w4qT (original URLs
/// too long).
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

/// Translate a ROYGBP colour point into an RGB colour point according to sampling the svg on wikipedia.
///
/// # Examples
///
/// ```
/// # use mandalas::util::roygbp_to_rgb;
/// assert_eq!(roygbp_to_rgb([0f64, 0f64, 0f64, 0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn roygbp_to_rgb(roygbp: [f64; 6]) -> [f64; 3] {
    // R = e11716
    // O = fc9019
    // Y = fcf219
    // G = 028226
    // B = 3032fe
    // P = 760087

    [roygbp[0] * 0.250836120401338 + roygbp[1] * 0.280936454849498 + roygbp[2] * 0.280936454849498 + roygbp[3] * 0.00222965440356745 +
     roygbp[4] * 0.0535117056856187 + roygbp[5] * 0.131549609810479,

     roygbp[0] * 0.0390492359932088 + roygbp[1] * 0.244482173174873 + roygbp[2] * 0.410865874363328 + roygbp[3] * 0.220713073005093 +
     roygbp[4] * 0.0848896434634975,

     roygbp[0] * 0.0440881763527054 + roygbp[1] * 0.0501002004008016 + roygbp[2] * 0.0501002004008016 + roygbp[3] * 0.0761523046092184 +
     roygbp[4] * 0.509018036072144 + roygbp[5] * 0.270541082164329]
}

/// Translate a PROYGBIV colour point into an RGB colour point according to sampling the svg on wikipedia.
///
/// # Examples
///
/// ```
/// # use mandalas::util::proygbiv_to_rgb;
/// assert_eq!(proygbiv_to_rgb([0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn proygbiv_to_rgb(proygbiv: [f64; 8]) -> [f64; 3] {
    // P = ff69b4
    // R = ff0000
    // O = ff8e00
    // Y = ffff00
    // G = 008e00
    // B = 00c0c0
    // I = 400098
    // V = 8e008e

    [proygbiv[0] * 0.207993474714519 +
     proygbiv[1] * 0.207993474714519 +
     proygbiv[2] * 0.207993474714519 +
     proygbiv[3] * 0.207993474714519 +
     proygbiv[6] * 0.0522022838499184 +
     proygbiv[7] * 0.115823817292007,

     proygbiv[0] * 0.125598086124402 +
     proygbiv[2] * 0.169856459330144 +
     proygbiv[3] * 0.305023923444976 +
     proygbiv[4] * 0.169856459330144 +
     proygbiv[5] * 0.229665071770335,

     proygbiv[0] * 0.27027027027027 +
     proygbiv[5] * 0.288288288288288 +
     proygbiv[6] * 0.228228228228228 +
     proygbiv[7] * 0.213213213213213]
}
