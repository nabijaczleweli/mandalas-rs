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
