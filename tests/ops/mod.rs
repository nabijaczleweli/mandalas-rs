mod points_to_generate;
mod init_image;

use image::{self, GenericImage, DynamicImage, ColorType};
use std::env::temp_dir;
use mandalas::ops;
use std::fs;


#[test]
fn filename_to_save() {
    assert_eq!(ops::filename_to_save((845, 91)), "mandala-845x91.png".to_string());
}

#[test]
fn save() {
    let mut path = temp_dir();
    let _ = fs::create_dir(&path);
    path.push("mandalas-test");
    let _ = fs::create_dir(&path);
    path.push("save-0");
    let _ = fs::create_dir(&path);

    ops::save(&DynamicImage::new_rgb8(1, 2), &path, "mandala.png");

    path.push("mandala.png");
    assert!(path.exists());

    let img = image::open(&path).unwrap();
    assert_eq!(img.dimensions(), (1, 2));
    assert_eq!(img.color(), ColorType::RGB(8));
}
