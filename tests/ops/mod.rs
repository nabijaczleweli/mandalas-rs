mod points_to_generate;
mod init_image;

use mandalas::ops;


#[test]
fn filename_to_save() {
    assert_eq!(&ops::filename_to_save((845, 91, 6322), 4213), "mandala-845x91x6322-04213.png");
}
