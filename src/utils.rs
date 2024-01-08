pub fn extract_digits_u8(mut num: u8) -> Vec<u8> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    digits
}

#[test]
fn test_digit_extraction() {
    assert_eq!(extract_digits_u8(123), [1, 2, 3]);
    assert_eq!(extract_digits_u8(254), [2, 5, 4]);
}
