use mandalas::util;

mod distance;


#[test]
fn separated_number() {
    assert_eq!(util::separated_number(1), "1");
    assert_eq!(util::separated_number(12), "12");
    assert_eq!(util::separated_number(123), "123");
    assert_eq!(util::separated_number(1234), "1'234");
    assert_eq!(util::separated_number(12345), "12'345");
    assert_eq!(util::separated_number(123456), "123'456");
    assert_eq!(util::separated_number(1234567), "1'234'567");
    assert_eq!(util::separated_number(12345678), "12'345'678");
    assert_eq!(util::separated_number(123456789), "123'456'789");
    assert_eq!(util::separated_number(1234567890), "1'234'567'890");
    assert_eq!(util::separated_number(12345678901), "12'345'678'901");
    assert_eq!(util::separated_number(123456789012), "123'456'789'012");
    assert_eq!(util::separated_number(1234567890123), "1'234'567'890'123");
    assert_eq!(util::separated_number(12345678901234), "12'345'678'901'234");
    assert_eq!(util::separated_number(123456789012345), "123'456'789'012'345");
    assert_eq!(util::separated_number(1234567890123456), "1'234'567'890'123'456");
    assert_eq!(util::separated_number(12345678901234567), "12'345'678'901'234'567");
    assert_eq!(util::separated_number(123456789012345678), "123'456'789'012'345'678");
    assert_eq!(util::separated_number(1234567890123456789), "1'234'567'890'123'456'789");
    assert_eq!(util::separated_number(12345678901234567890), "12'345'678'901'234'567'890");
}
