// NONOS Operating System (AGPL-3.0-or-later)
//! Hardening proofs: no input, however malformed, may make a parser panic,
//! read out of bounds or hang; each runs over hundreds of thousands of
//! pseudo-random inputs. The AP controls every beacon byte. The CCMP and
//! EAPOL ones now run against the core, in `nonos_wifi_core_proofs`.

use crate::dot11::parse::{find_ie, parse_beacon};

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn fuzz_buf(s: &mut u64, max_len: usize) -> Vec<u8> {
    let len = (xorshift(s) as usize) % (max_len + 1);
    let mut b = vec![0u8; len];
    for x in b.iter_mut() {
        *x = xorshift(s) as u8;
    }
    b
}

#[test]
fn beacon_parser_never_panics_on_arbitrary_frames() {
    let mut s = 0x1234_5678_9abc_def0;
    for _ in 0..400_000 {
        let buf = fuzz_buf(&mut s, 80);
        let _ = parse_beacon(&buf);
    }
}

#[test]
fn find_ie_terminates_and_never_panics() {
    let mut s = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..400_000 {
        let tags = fuzz_buf(&mut s, 64);
        let id = xorshift(&mut s) as u8;
        let _ = find_ie(&tags, id);
    }
}
