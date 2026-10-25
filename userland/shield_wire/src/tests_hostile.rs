// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile bytes for the shield's wire (the owner's rule 8): frames of every
//! short length and longer, zero and 0xff fills, and field text the parsers
//! must refuse without a panic.
use super::*;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let [low, ..] = x.to_le_bytes();
            low
        })
        .collect()
}

#[test]
fn no_frame_or_field_panics_on_hostile_bytes() {
    for n in (0..300).chain([BODY_MAX, MESSAGE_MAX, MESSAGE_MAX + 1]) {
        for b in [bytes(n as u64 + 1, n), vec![0; n], vec![0xff; n]] {
            let _ = (decode_request(&b), decode_reply(&b));
            let text = String::from_utf8_lossy(&b);
            let _ = (field(&text, "raw"), fields(&text, "handed").count(), unhex(&text));
            let words: Vec<&str> = text.split('\n').collect();
            let _ = FreshAsk::parse(&words);
        }
    }
    for odd in ["0x", "0x0", "0xzz", "0x\u{e9}a", "1|2|3", "0x00\n0x00"] {
        let _ = (unhex(odd), FreshAsk::parse(&[odd; 9]));
    }
    assert_eq!(unhex("0x"), Some(Vec::new()));
    assert_eq!(unhex("0x\u{e9}a"), None, "a multi-byte character is not hex");
}
