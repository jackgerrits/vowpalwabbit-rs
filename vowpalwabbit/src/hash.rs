use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

fn fmix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    h
}

#[allow(arithmetic_overflow)]
pub fn uniform_hash(key: &[u8], seed: u64) -> u64 {
    let num_blocks = key.len() / 4;
    let mut h1 = seed as u32;

    const CONSTANT1: u32 = 0xcc9e_2d51;
    const CONSTANT2: u32 = 0x1b87_3593;

    // Body
    for i in 0..num_blocks {
        let mut b = Cursor::new(&key[(i * 4)..(i * 4) + 4]);
        let mut current_block = b.read_u32::<LittleEndian>().unwrap();

        current_block = current_block.wrapping_mul(CONSTANT1);
        current_block = current_block.rotate_left(15);
        current_block = current_block.wrapping_mul(CONSTANT2);

        h1 ^= current_block;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe654_6b64);
    }

    // Tail
    let mut k1: u32 = 0;
    let tail = &key[num_blocks * 4..];

    if !tail.is_empty() {
        if tail.len() >= 3 {
            k1 ^= u32::from(tail[2]).wrapping_shl(16);
        }

        if tail.len() >= 2 {
            k1 ^= u32::from(tail[1]).wrapping_shl(8);
        }

        k1 ^= u32::from(tail[0]);
        k1 = k1.wrapping_mul(CONSTANT1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(CONSTANT2);
        h1 ^= k1;
    }

    // Finalization
    h1 ^= key.len() as u32;
    u64::from(fmix(h1))
}

// Test truth values calculated using C++ implementation.
#[test]
fn fmix_tests() {
    assert_eq!(fmix(0), 0);
    assert_eq!(fmix(1), 1364076727);
    assert_eq!(fmix(5), 3423425485);
    assert_eq!(fmix(2147483647), 4190899880);
    assert_eq!(fmix(4294967295), 2180083513);
}

#[test]
fn hash_tests_zero_seed() {
    assert_eq!(uniform_hash(b"t", 0), 3397902157);
    assert_eq!(uniform_hash(b"te", 0), 3988319771);
    assert_eq!(uniform_hash(b"tes", 0), 196677210);
    assert_eq!(uniform_hash(b"test", 0), 3127628307);
    assert_eq!(uniform_hash(b"tested", 0), 2247989476);
    assert_eq!(
        uniform_hash(b"8hv20cjwicnsj vw m000'.'.][][]...!!@3", 0),
        4212741639
    );
}

#[test]
fn hash_tests_nonzero_seed() {
    assert_eq!(uniform_hash(b"t", 25436347), 960607349);
    assert_eq!(uniform_hash(b"te", 25436347), 2834341637);
    assert_eq!(uniform_hash(b"tes", 25436347), 1163171263);
    assert_eq!(uniform_hash(b"tested", 25436347), 3592599130);
    assert_eq!(
        uniform_hash(b"8hv20cjwicnsj vw m000'.'.][][]...!!@3", 25436347),
        2503360452
    );
}
