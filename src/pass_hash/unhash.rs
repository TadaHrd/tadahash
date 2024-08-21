use crate::tadahash::tadahash;

pub fn pass_unhash(password: u128, data: &mut [u8]) {
    let pass_bytes = password.to_be_bytes();
    let pass_bytes = tadahash(&pass_bytes).to_be_bytes();

    let mut last_byte = 0;

    for byte in data.iter_mut() {
        let temp = *byte;

        *byte = byte.wrapping_sub(pass_bytes[15]);

        *byte = byte.reverse_bits();

        *byte ^= pass_bytes[14].reverse_bits();

        *byte = byte.wrapping_add(pass_bytes[13]);
        *byte = byte.reverse_bits();

        *byte ^= pass_bytes[12].wrapping_add(7);

        *byte = byte.rotate_left(pass_bytes[11] as u32);
        *byte = byte.rotate_right(pass_bytes[10] as u32);

        *byte ^= last_byte;

        *byte = byte.wrapping_sub(last_byte);

        last_byte = temp;
    }
    for byte in data.iter_mut().skip(1).step_by(2) {
        *byte = byte.wrapping_sub(pass_bytes[9]);
        *byte = byte.rotate_left(pass_bytes[8] as u32);

        *byte = !*byte;

        *byte ^= pass_bytes[7];

        *byte = byte.wrapping_sub(pass_bytes[6]);
        *byte = byte.wrapping_add(pass_bytes[5]);
    }
    for byte in data.iter_mut().step_by(2) {
        *byte = byte.wrapping_sub(pass_bytes[4]);
        *byte = byte.rotate_right(pass_bytes[3] as u32);

        *byte = !*byte;

        *byte ^= pass_bytes[2];

        *byte = byte.wrapping_sub(pass_bytes[1]);
        *byte = byte.wrapping_add(pass_bytes[0]);
    }
}
