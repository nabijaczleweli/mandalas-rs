use image::{GenericImage, ColorType};
use mandalas::ops::init_image;


#[test]
fn size() {
    let i = init_image((1234, 5678));
    assert_eq!(i.width(), 1234);
    assert_eq!(i.height(), 5678);
}

#[test]
fn colour_type() {
    assert_eq!(init_image((1234, 5678)).color(), ColorType::RGB(8));
}
