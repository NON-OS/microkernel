// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The NTP client believes a reply only when it answers its own request from a
//! synchronized server, and reads the time right on both sides of 2036.

use crate::sntp::{build_request, parse_reply, SntpError};

const NONCE: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// A good server reply to `NONCE` with transmit seconds `secs`.
fn reply(secs: u32) -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0] = 0x24; /* leap 0, version 4, mode 4 */
    b[1] = 2;
    b[24..32].copy_from_slice(&NONCE);
    b[40..44].copy_from_slice(&secs.to_be_bytes());
    b
}

#[test]
fn the_request_is_client_mode_v4_and_carries_the_nonce() {
    let r = build_request(NONCE);
    assert_eq!((r[0] & 0x38, r[0] & 0x07), (0x20, 0x03));
    assert_eq!(r[40..48], NONCE);
}

#[test]
fn a_good_reply_gives_unix_milliseconds() {
    assert_eq!(parse_reply(&reply(3_913_056_000), &NONCE), Ok(1_704_067_200_000));
}

#[test]
fn a_reply_that_does_not_echo_the_nonce_is_not_ours() {
    let mut b = reply(3_913_056_000);
    b[31] ^= 1;
    assert_eq!(parse_reply(&b, &NONCE), Err(SntpError::NotOurs));
    assert_eq!(parse_reply(&reply(3_913_056_000), &[0; 8]), Err(SntpError::NotOurs));
}

#[test]
fn an_unsynchronized_server_is_refused() {
    let mut b = reply(3_913_056_000);
    b[0] |= 0xc0; /* leap indicator 3 */
    assert_eq!(parse_reply(&b, &NONCE), Err(SntpError::Unsynchronized));
}

#[test]
fn wrong_shapes_are_refused() {
    assert_eq!(parse_reply(&[0u8; 47], &NONCE), Err(SntpError::Malformed));
    let mut client = reply(3_913_056_000);
    client[0] = 0x23;
    assert_eq!(parse_reply(&client, &NONCE), Err(SntpError::NotServer));
    let mut v1 = reply(3_913_056_000);
    v1[0] = 0x0c;
    assert_eq!(parse_reply(&v1, &NONCE), Err(SntpError::NotServer));
    let mut kiss = reply(3_913_056_000);
    kiss[1] = 0;
    assert_eq!(parse_reply(&kiss, &NONCE), Err(SntpError::NotServer));
    assert_eq!(parse_reply(&reply(0), &NONCE), Err(SntpError::BadTimestamp));
}

#[test]
fn the_2036_rollover_reads_forward() {
    /* The last second of era 0, then the first seconds of era 1. */
    assert_eq!(parse_reply(&reply(u32::MAX), &NONCE), Ok(2_085_978_495_000));
    assert_eq!(parse_reply(&reply(0x0000_0001), &NONCE), Ok(2_085_978_497_000));
    assert_eq!(parse_reply(&reply(0x7fff_ffff), &NONCE), Ok(4_233_462_143_000));
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/*
 * Over random replies of every length, about half carrying the nonce, the
 * parser never panics, and every time it believes one, each check the lead's
 * fix put in holds and the time is the transmit seconds read on the 2036
 * rollover rule.
 */
#[test]
fn a_believed_reply_always_passed_every_check() {
    let mut s = 0x5A7E_0001u32;
    for _ in 0..100_000 {
        let len =
            if xorshift(&mut s).is_multiple_of(8) { (xorshift(&mut s) % 64) as usize } else { 48 };
        let mut b: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        if len == 48 && xorshift(&mut s).is_multiple_of(2) {
            b[24..32].copy_from_slice(&NONCE);
        }
        let Ok(ms) = parse_reply(&b, &NONCE) else { continue };
        assert_eq!(b.len(), 48);
        let (leap, version, mode, stratum) = (b[0] >> 6, (b[0] >> 3) & 7, b[0] & 7, b[1]);
        assert!(mode == 4 && (3..=4).contains(&version) && (1..=15).contains(&stratum));
        assert!(leap != 3 && b[24..32] == NONCE);
        let secs = u64::from(u32::from_be_bytes([b[40], b[41], b[42], b[43]]));
        assert_ne!(secs, 0);
        let ntp = if secs >= 1 << 31 { secs } else { secs + (1 << 32) };
        assert!(ntp >= 2_208_988_800, "a time before 1970 was believed");
        assert_eq!(ms, (ntp - 2_208_988_800) * 1000);
    }
}

/*
 * A transmit time from 1968 or 1969 (top bit set, below the unix epoch) was
 * subtracted unchecked: a debug build panicked, and the release build wrapped
 * to a clock thousands of years off and set it.
 */
#[test]
fn a_transmit_time_before_1970_is_refused() {
    for secs in [0x8000_0000u32, 0x83AA_7E7F] {
        assert_eq!(parse_reply(&reply(secs), &NONCE), Err(SntpError::BadTimestamp), "{secs:#x}");
    }
    assert_eq!(parse_reply(&reply(0x83AA_7E80), &NONCE), Ok(0), "the epoch itself is time zero");
}
