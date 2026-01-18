use testing;
mod common;

#[test]
fn adds_two_ma_boi() {
    common::setup();
    assert_eq!(testing::add_two(2), 4);
}