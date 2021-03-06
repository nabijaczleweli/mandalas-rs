//! Various utility functions.


use itertools::Itertools;
use std::u8;


pub static FLAG_ACE: &[[f64; 3]] = &[[0f64, 0f64, 0f64],
                                     [(0xA3 as f64) / (0xFF as f64), (0xA3 as f64) / (0xFF as f64), (0xA3 as f64) / (0xFF as f64)],
                                     [1f64, 1f64, 1f64],
                                     [(0x80 as f64) / (0xFF as f64), 0f64, (0x80 as f64) / (0xFF as f64)]];


/// Calculate the distance between two specified 3-dimensional points.
pub fn distance(from: (usize, usize, usize), to: (usize, usize, usize)) -> f64 {
    let x = (from.0 as isize) - (to.0 as isize);
    let y = (from.1 as isize) - (to.1 as isize);
    let z = (from.2 as isize) - (to.2 as isize);
    ((x * x + y * y + z * z) as f64).sqrt()
    // (x as f64 / y as f64).powi(z as i32)
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
    let rec_k = 1f64 - cmyk[3];
    [(1f64 - cmyk[0]) * rec_k, (1f64 - cmyk[1]) * rec_k, (1f64 - cmyk[2]) * rec_k]
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

    [proygbiv[0] * 0.207993474714519 + proygbiv[1] * 0.207993474714519 + proygbiv[2] * 0.207993474714519 + proygbiv[3] * 0.207993474714519 +
     proygbiv[6] * 0.0522022838499184 + proygbiv[7] * 0.115823817292007,

     proygbiv[0] * 0.125598086124402 + proygbiv[2] * 0.169856459330144 + proygbiv[3] * 0.305023923444976 + proygbiv[4] * 0.169856459330144 +
     proygbiv[5] * 0.229665071770335,

     proygbiv[0] * 0.27027027027027 + proygbiv[5] * 0.288288288288288 + proygbiv[6] * 0.228228228228228 + proygbiv[7] * 0.213213213213213]
}

/// Translate a RPPDIBLCTGLLYAODBGB colour point into an RGB colour point according to
/// [this PNG](https://learntocodewith.me/wp-content/uploads/2016/01/material-palette.png).
///
/// # Examples
///
/// ```
/// # use mandalas::util::rppdiblctgllyaodbgb_to_rgb;
/// assert_eq!(rppdiblctgllyaodbgb_to_rgb(
///     [0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
///      0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
///      0f64, 0f64, 0f64, 0f64, 0f64]),
///     [0f64, 0f64, 0f64]);
/// ```
pub fn rppdiblctgllyaodbgb_to_rgb(rppdiblctgllyaodbgb: [f64; 19]) -> [f64; 3] {
    // R = f34634
    // P = e81b62
    // P = 9c00b0
    // D = 6922b7
    // I = 4544b5
    // B = 328af4
    // L = 229ff5
    // C = 11b8d3
    // T = 009486
    // G = 48b14b
    // L = 86c642
    // L = c8e028
    // Y = fcf025
    // A = fdc500
    // O = fd9b00
    // D = fe5a1c
    // B = 775648
    // G = 9c9c9c
    // B = 607a89

    [rppdiblctgllyaodbgb[0] * 0.0901669758812616 + rppdiblctgllyaodbgb[1] * 0.0860853432282004 + rppdiblctgllyaodbgb[2] * 0.0578849721706865 +
     rppdiblctgllyaodbgb[3] * 0.038961038961039 + rppdiblctgllyaodbgb[4] * 0.0256029684601113 + rppdiblctgllyaodbgb[5] * 0.0185528756957328 +
     rppdiblctgllyaodbgb[6] * 0.0126159554730983 +
     rppdiblctgllyaodbgb[7] * 0.00630797773654917 + rppdiblctgllyaodbgb[9] * 0.0267161410018553 + rppdiblctgllyaodbgb[10] * 0.049721706864564 +
     rppdiblctgllyaodbgb[11] * 0.0742115027829314 +
     rppdiblctgllyaodbgb[12] * 0.0935064935064935 + rppdiblctgllyaodbgb[13] * 0.0938775510204082 + rppdiblctgllyaodbgb[14] * 0.0938775510204082 +
     rppdiblctgllyaodbgb[15] * 0.0942486085343228 +
     rppdiblctgllyaodbgb[16] * 0.0441558441558442 + rppdiblctgllyaodbgb[17] * 0.0578849721706865 + rppdiblctgllyaodbgb[18] * 0.0356215213358071,

     rppdiblctgllyaodbgb[0] * 0.0283057015770319 + rppdiblctgllyaodbgb[1] * 0.0109179134654266 + rppdiblctgllyaodbgb[3] * 0.0137484836231298 +
     rppdiblctgllyaodbgb[4] * 0.0274969672462596 + rppdiblctgllyaodbgb[5] * 0.0558026688232915 + rppdiblctgllyaodbgb[6] * 0.0642943792964011 +
     rppdiblctgllyaodbgb[7] * 0.0744035584310554 + rppdiblctgllyaodbgb[8] * 0.0598463404771533 + rppdiblctgllyaodbgb[9] * 0.0715729882733522 +
     rppdiblctgllyaodbgb[10] * 0.0800646987464618 + rppdiblctgllyaodbgb[11] * 0.0905782450465022 + rppdiblctgllyaodbgb[12] * 0.097048119692681 +
     rppdiblctgllyaodbgb[13] * 0.0796603315810756 +
     rppdiblctgllyaodbgb[14] * 0.0626769106348564 + rppdiblctgllyaodbgb[15] * 0.0363930448847554 +
     rppdiblctgllyaodbgb[16] * 0.0347755762232107 + rppdiblctgllyaodbgb[17] * 0.0630812778002426 + rppdiblctgllyaodbgb[18] * 0.0493327941771128,

     rppdiblctgllyaodbgb[0] * 0.0243559718969555 + rppdiblctgllyaodbgb[1] * 0.0459016393442623 + rppdiblctgllyaodbgb[2] * 0.0824355971896956 +
     rppdiblctgllyaodbgb[3] * 0.0857142857142857 + rppdiblctgllyaodbgb[4] * 0.0847775175644028 + rppdiblctgllyaodbgb[5] * 0.114285714285714 +
     rppdiblctgllyaodbgb[6] * 0.114754098360656 +
     rppdiblctgllyaodbgb[7] * 0.0988290398126464 + rppdiblctgllyaodbgb[8] * 0.0627634660421546 + rppdiblctgllyaodbgb[9] * 0.0351288056206089 +
     rppdiblctgllyaodbgb[10] * 0.0309133489461358 +
     rppdiblctgllyaodbgb[11] * 0.0187353629976581 + rppdiblctgllyaodbgb[12] * 0.0173302107728337 + rppdiblctgllyaodbgb[15] * 0.0131147540983607 +
     rppdiblctgllyaodbgb[16] * 0.0337236533957845 +
     rppdiblctgllyaodbgb[17] * 0.0730679156908665 + rppdiblctgllyaodbgb[18] * 0.0641686182669789]
}

/// Translate a PRGB1555 colour point into an RGB colour point.
///
/// The P bit is the lowest bit.
///
/// # Examples
///
/// ```
/// # use mandalas::util::prgb1555_to_rgb;
/// assert_eq!(prgb1555_to_rgb([0f64, 0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn prgb1555_to_rgb(prgb1555: [f64; 4]) -> [f64; 3] {
    const MAX_COLOUR: f64 = u8::MAX as f64;

    let p_mask = ((prgb1555[0] * MAX_COLOUR) as u8 & 1) << 7;

    [((((prgb1555[1] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR,
     ((((prgb1555[2] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR,
     ((((prgb1555[3] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR]
}

/// Translate a PRGB1555 colour point into an RGB colour point.
///
/// The P bit is equal to bitcount(P) >= 6.
///
/// # Examples
///
/// ```
/// # use mandalas::util::prgb1555_least6_to_rgb;
/// assert_eq!(prgb1555_least6_to_rgb([0f64, 0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn prgb1555_least6_to_rgb(prgb1555: [f64; 4]) -> [f64; 3] {
    const MAX_COLOUR: f64 = u8::MAX as f64;

    let p_mask = ((((prgb1555[0] * MAX_COLOUR) as u8).count_ones() >= 6) as u8) << 7;

    [((((prgb1555[1] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR,
     ((((prgb1555[2] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR,
     ((((prgb1555[3] * MAX_COLOUR) as u8 >> 3) << 2) | p_mask) as f64 / MAX_COLOUR]
}

/// Translate a colour point and alpha into an RGB colour point from the specified flag colourset.
///
/// # Examples
///
/// ```
/// # use mandalas::util::{FLAG_ACE, flag_hard_alpha_to_rgb};
/// assert_eq!(flag_hard_alpha_to_rgb(FLAG_ACE, [0f64, 1f64]), FLAG_ACE[0]);
/// assert_eq!(flag_hard_alpha_to_rgb(FLAG_ACE, [0.2 , 1f64]), FLAG_ACE[0]);
/// assert_eq!(flag_hard_alpha_to_rgb(FLAG_ACE, [0.4 , 1f64]), FLAG_ACE[1]);
/// assert_eq!(flag_hard_alpha_to_rgb(FLAG_ACE, [0.4 , 0.5 ]), [FLAG_ACE[1][0] * 0.5, FLAG_ACE[1][1] * 0.5, FLAG_ACE[1][2] * 0.5]);
/// ```
pub fn flag_hard_alpha_to_rgb(flag: &[[f64; 3]], id_alpha: [f64; 2]) -> [f64; 3] {
    let colour = flag[(id_alpha[0] / (1f64 / (flag.len() as f64))).floor().min((flag.len() - 1) as f64) as usize];

    [colour[0] * id_alpha[1], colour[1] * id_alpha[1], colour[2] * id_alpha[1]]
}




/*
/// Translate a PROYGBIV colour point into an RGB colour point according to sampling the svg on wikipedia.
///
/// # Examples
///
/// ```
/// # use mandalas::util::proygbiv_to_rgb;
/// assert_eq!(proygbiv_to_rgb([0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64]), [0f64, 0f64, 0f64]);
/// ```
pub fn lab_to_rgb(lab: [f64; 3]) -> [f64; 3] {
    float var_Y = ( l_s + 16. ) / 116.;
    float var_X = a_s / 500. + var_Y;
    float var_Z = var_Y - b_s / 200.;

    if ( pow(var_Y,3) > 0.008856 ) var_Y = pow(var_Y,3);
    else                      var_Y = ( var_Y - 16. / 116. ) / 7.787;
    if ( pow(var_X,3) > 0.008856 ) var_X = pow(var_X,3);
    else                      var_X = ( var_X - 16. / 116. ) / 7.787;
    if ( pow(var_Z,3) > 0.008856 ) var_Z = pow(var_Z,3);
    else                      var_Z = ( var_Z - 16. / 116. ) / 7.787;

    float X = 95.047 * var_X ;    //ref_X =  95.047     Observer= 2°, Illuminant= D65
    float Y = 100.000 * var_Y  ;   //ref_Y = 100.000
    float Z = 108.883 * var_Z ;    //ref_Z = 108.883


    var_X = X / 100. ;       //X from 0 to  95.047      (Observer = 2°, Illuminant = D65)
    var_Y = Y / 100. ;       //Y from 0 to 100.000
    var_Z = Z / 100. ;      //Z from 0 to 108.883

    float var_R = var_X *  3.2406 + var_Y * -1.5372 + var_Z * -0.4986;
    float var_G = var_X * -0.9689 + var_Y *  1.8758 + var_Z *  0.0415;
    float var_B = var_X *  0.0557 + var_Y * -0.2040 + var_Z *  1.0570;

    if ( var_R > 0.0031308 ) var_R = 1.055 * pow(var_R , ( 1 / 2.4 ))  - 0.055;
    else                     var_R = 12.92 * var_R;
    if ( var_G > 0.0031308 ) var_G = 1.055 * pow(var_G , ( 1 / 2.4 ) )  - 0.055;
    else                     var_G = 12.92 * var_G;
    if ( var_B > 0.0031308 ) var_B = 1.055 * pow( var_B , ( 1 / 2.4 ) ) - 0.055;
    else                     var_B = 12.92 * var_B;

    R = var_R * 255.;
    G = var_G * 255.;
    B = var_B * 255.;
}

void LAB2RGB(int L, int a, int b, unsigned char & R, unsigned char & G, unsigned char & B)
{
    float X, Y, Z, fX, fY, fZ;
    int RR, GG, BB;

    fY = pow((L + 16.0) / 116.0, 3.0);
    if (fY < 0.008856)
        fY = L / 903.3;
    Y = fY;

    if (fY > 0.008856)
        fY = powf(fY, 1.0/3.0);
    else
        fY = 7.787 * fY + 16.0/116.0;

    fX = a / 500.0 + fY;
    if (fX > 0.206893)
        X = powf(fX, 3.0);
    else
        X = (fX - 16.0/116.0) / 7.787;

    fZ = fY - b /200.0;
    if (fZ > 0.206893)
        Z = powf(fZ, 3.0);
    else
        Z = (fZ - 16.0/116.0) / 7.787;

    X *= (0.950456 * 255);
    Y *=             255;
    Z *= (1.088754 * 255);

    RR =  (int)(3.240479*X - 1.537150*Y - 0.498535*Z + 0.5);
    GG = (int)(-0.969256*X + 1.875992*Y + 0.041556*Z + 0.5);
    BB =  (int)(0.055648*X - 0.204043*Y + 1.057311*Z + 0.5);

    R = (unsigned char)(RR < 0 ? 0 : RR > 255 ? 255 : RR);
    G = (unsigned char)(GG < 0 ? 0 : GG > 255 ? 255 : GG);
    B = (unsigned char)(BB < 0 ? 0 : BB > 255 ? 255 : BB);

    //printf("Lab=(%f,%f,%f) ==> RGB(%f,%f,%f)\n",L,a,b,*R,*G,*B);
}*/
