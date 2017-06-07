use mandalas::util::distance;


#[test]
fn simple() {
    assert_eq!(distance((0, 0, 0), (3, 4, 0)), 5.0);
    assert_eq!(distance((0, 0, 0), (6, 8, 0)), 10.0);
    assert_eq!(distance((0, 0, 0), (0, 8, 6)), 10.0);
}

#[test]
fn inverse() {
    assert_eq!(distance((0, 4, 0), (3, 0, 0)), 5.0);
    assert_eq!(distance((0, 8, 0), (6, 0, 0)), 10.0);
    assert_eq!(distance((0, 0, 0), (9, 12, 8)), 17.0);
}
