/// Very simple 8-bit non-cryptographic hashing function.
pub fn simple_hash(byte: u8) -> u8 {
    // prime number   vvv
    byte.wrapping_mul(179)
}