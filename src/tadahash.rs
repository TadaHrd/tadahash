/// Non-cryptographic 128-bit hashing function.
pub fn tadahash(data: &[u8]) -> u128 {
    // prime
    let mut hash: u128 = 79_876_892_682_460_138_982_994_621_485_704_188_421;

    let mut last_byte = 0;

    for &byte in data {
        hash = hash.wrapping_mul(byte as u128);

        //                                prime
        //                                 vv
        hash ^= (byte as u128).rotate_left(73).wrapping_neg();

        hash = hash.wrapping_sub(last_byte as u128);

        last_byte = byte;
    }

    // prime
    hash.wrapping_mul(78_591_149_900_362_871_890_310_893_430_354_426_911)
}
