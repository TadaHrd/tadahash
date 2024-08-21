use crate::tadahash::tadahash;

pub fn pass_hash(password: u128, data: &mut [u8]) {
    // prime
    // let prime = 179;
    let mut last_byte = 0;

    let pass_bytes = password.to_be_bytes();
    let pass_bytes = tadahash(&pass_bytes).to_be_bytes();

    for byte in data.iter_mut().step_by(2) {
        *byte = byte.wrapping_sub(pass_bytes[0]);
        *byte = byte.wrapping_add(pass_bytes[1]);

        *byte ^= pass_bytes[2];

        *byte = !*byte;

        *byte = byte.rotate_left(pass_bytes[3] as u32);
        *byte = byte.wrapping_add(pass_bytes[4]);
    }
    for byte in data.iter_mut().skip(1).step_by(2) {
        *byte = byte.wrapping_sub(pass_bytes[5]);
        *byte = byte.wrapping_add(pass_bytes[6]);

        *byte ^= pass_bytes[7];

        *byte = !*byte;

        *byte = byte.rotate_right(pass_bytes[8] as u32);
        *byte = byte.wrapping_add(pass_bytes[9]);
    }
    for byte in data.iter_mut() {
        *byte = byte.wrapping_add(last_byte);

        *byte ^= last_byte;

        *byte = byte.rotate_left(pass_bytes[10] as u32);
        *byte = byte.rotate_right(pass_bytes[11] as u32);

        //        best prime in the world    v
        *byte ^= pass_bytes[12].wrapping_add(7);

        *byte = byte.reverse_bits();
        *byte = byte.wrapping_sub(pass_bytes[13]);

        *byte ^= pass_bytes[14].reverse_bits();

        *byte = byte.reverse_bits();

        *byte = byte.wrapping_add(pass_bytes[15]);

        last_byte = *byte;
    }
}
