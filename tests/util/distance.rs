use mandalas::util::distance;


#[test]
fn simple() {
    assert_eq!(distance((0, 0), (3, 4)), 5.0);
    assert_eq!(distance((0, 0), (6, 8)), 10.0);
}

#[test]
fn inverse() {
    assert_eq!(distance((0, 4), (3, 0)), 5.0);
    assert_eq!(distance((0, 8), (6, 0)), 10.0);
    assert_eq!(distance((0, 0), (9, 12)), 15.0);
}
