//! Various utility functions.


use itertools::Itertools;


/// Calculate the distance between two specified points.
///
/// # Examples
///
/// Calculate the lengths of the hypotenuses of various Egiptian triangles.
///
/// ```
/// # use mandalas::util::distance;
/// assert_eq!(distance((0, 0), (3, 4)), 5.0);
/// assert_eq!(distance((0, 0), (6, 8)), 10.0);
/// assert_eq!(distance((0, 0), (9, 12)), 15.0);
/// assert_eq!(distance((0, 0), (16, 12)), 20.0);
/// ```
pub fn distance(from: (usize, usize), to: (usize, usize)) -> f64 {
    let mut x = [from.0, to.0];
    let mut y = [from.1, to.1];
    x.sort();
    y.sort();

    let x = x[1] - x[0];
    let y = y[1] - y[0];
    ((x * x + y * y) as f64).sqrt()
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
