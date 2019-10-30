use image::{GenericImageView, ColorType};
use mandalas::ops::init_image;


#[test]
fn size() {
    let imgs = init_image((123, 456, 78));
    assert_eq!(imgs.len(), 78);
    for img in &imgs {
        assert_eq!(img.dimensions(), (123, 456));
    }
}

#[test]
fn colour_type() {
    let imgs = init_image((123, 456, 78));
    for img in &imgs {
        assert_eq!(img.color(), ColorType::RGB(8));
    }
}
