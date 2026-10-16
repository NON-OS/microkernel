// A test runs hostile input through each parser; any panic fails it.

//! What a lander's onion service can answer: no reading of it may panic or loop.

use super::reply::{handed, serves, state, takes_submitter_fee};

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
        r#"{"queued":true,"queued":false}"#.to_string(),
        r#"{"pools":["#.to_string(),
        r#"{"fee":"\u0000\u202e"}"#.to_string(),
        "{\"why\":\"".to_string() + &"\\".repeat(10_000),
        "[".repeat(10_000),
    ]
    .to_vec()
}

#[test]
fn no_relay_reply_reading_panics_on_hostile_answers() {
    for b in hostile(&misleading()) {
        let s = String::from_utf8_lossy(&b);
        for status in [0, 200, 202, 400, 429, 500, u16::MAX] {
            let _ = handed(status, &s);
        }
        let _ = (state(&s), serves(&s, "0x01"), serves(&s, ""), takes_submitter_fee(&s));
    }
}
