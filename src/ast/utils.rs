pub fn is_letter(char: u8) -> bool {
    char >= b'a' && char <= b'z' || char >= b'A' && char <= b'Z'
}
pub fn is_digit(char: u8) -> bool {
    char >= b'0' && char <= b'9'
}
