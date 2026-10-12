// A test runs hostile input through each parser; any panic fails it.
#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! Text a relay, a publisher record or a pasted signature can carry: no reading of it may
//! panic or loop.

use super::publish::Published;
use super::relay_body::limbs;
use super::rewards::parse_signature;

/// Bytes from a fixed seed, so a failure names the case that broke.
fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x.to_le_bytes()[0]
        })
        .collect()
}

/// Random bytes, runs of the characters a scanner keys on, and replies built to mislead.
fn hostile(misleading: &[String]) -> Vec<Vec<u8>> {
    let lens = (0..200usize).chain([1024, 70_000]);
    let random = lens.flat_map(|n| [bytes(n as u64 + 7, n), vec![b'{'; n], vec![b'"'; n]]);
    random.chain(misleading.iter().map(|s| s.clone().into_bytes())).collect()
}

fn misleading() -> Vec<String> {
    [
        "0x".to_string() + &"f".repeat(129),
        "0x".to_string() + &"f".repeat(130),
        "0x".to_string() + &"\u{e9}".repeat(65),
        "[".to_string() + &"18446744073709551616,".repeat(37) + "1]",
        "[".to_string() + &"1,".repeat(100_000) + "1]",
        "\n".repeat(10_000),
    ]
    .to_vec()
}

#[test]
fn no_wallet_text_reading_panics_on_hostile_input() {
    for b in hostile(&misleading()) {
        let s = String::from_utf8_lossy(&b);
        let _ = (parse_signature(&s), limbs(&s), Published::from_text(&s));
    }
}
