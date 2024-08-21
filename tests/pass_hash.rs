use tadahash::pass_hash::{pass_hash as pass_hash_, pass_unhash};

static DATA: &[u8] = include_bytes!("input.bin");
static PASSWORDS: [u128; 10] = [
    0xAA55AA55AA55AA55AA55AA55AA55AA55,
    0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    0x00000000000000000000000000000000,
    0x10000000000000000000000000000000,
    0x00000000000000000000000000000001,
    0xDEADBEEFDEADDEADBEEFDEADBEEFDEAD,
    0x0123465789ABCDEF0123465789ABCDEF,
    0x55555555555555555555555555555555,
    0x11111111111111111111111111111111,
    0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0,
];

fn do_test(pass: u128) {
    let mut vec: Vec<u8> = DATA.to_owned();

    pass_hash_(pass, &mut vec);
    assert!(vec != DATA, "hasher didn't hash");

    pass_unhash(pass, &mut vec);
    assert!(vec == DATA, "hasher didn't unhash properly {pass:X}");

    pass_hash_(pass, &mut vec);
    pass_unhash(pass.wrapping_add(1), &mut vec);
    assert!(
        vec != DATA,
        "`pass + 1` can't hash to the same value as `pass`"
    )
}

#[test]
fn pass_hash() {
    for pass in PASSWORDS {
        do_test(pass);
    }
}
