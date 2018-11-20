use super::*;

#[test]
fn test_random_hand() {
    assert_eq!("hand", format!("{}", Hand::random()))
}
