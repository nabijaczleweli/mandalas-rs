use image::{GenericImage, ColorType};
use mandalas::ops::init_image;


#[test]
fn size() {
    assert_eq!(init_image((1234, 5678)).dimensions(), (1234, 5678));
}

#[test]
fn colour_type() {
    assert_eq!(init_image((1234, 5678)).color(), ColorType::RGB(8));
}
