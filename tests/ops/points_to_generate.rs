use mandalas::ops::points_to_generate;


#[test]
fn exact() {
    assert_eq!(points_to_generate((10, 10, 1)), 10 * 10 * 1 * 10);
    assert_eq!(points_to_generate((100, 10, 10)), 100 * 10 * 10 * 10);
    assert_eq!(points_to_generate((100, 100, 100)), 100 * 100 * 100 * 10);
    assert_eq!(points_to_generate((1000, 100, 10000)), 1000 * 100 * 10000 * 10);
    assert_eq!(points_to_generate((100, 1000, 1000000)), 100 * 1000 * 1000000 * 10);
}

#[test]
fn low() {
    assert_eq!(points_to_generate((15, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((14, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((13, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((12, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((11, 10, 1)), 1000 * 10);
}

#[test]
fn high() {
    assert_eq!(points_to_generate((19, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((18, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((17, 10, 1)), 1000 * 10);
    assert_eq!(points_to_generate((16, 10, 1)), 1000 * 10);
}
