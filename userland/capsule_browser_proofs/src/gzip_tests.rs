// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for gzip members (RFC 1952): every member's trailer is
//! present and its CRC32 and ISIZE match the bytes it inflated to.

use alloc::vec::Vec;
use nonos_inflate::gunzip;

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
        }
    }
    !c
}

fn member_with(payload: &[u8], crc: u32, isize: u32) -> Vec<u8> {
    let mut m = alloc::vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff, 1];
    let n = payload.len() as u16;
    m.extend_from_slice(&n.to_le_bytes());
    m.extend_from_slice(&(!n).to_le_bytes());
    m.extend_from_slice(payload);
    m.extend_from_slice(&crc.to_le_bytes());
    m.extend_from_slice(&isize.to_le_bytes());
    m
}

fn member(payload: &[u8]) -> Vec<u8> {
    member_with(payload, crc32(payload), payload.len() as u32)
}

#[test]
fn decodes_every_member_in_order() {
    let mut gz = member(b"hello, ");
    gz.extend(member(b"world"));
    assert_eq!(gunzip(&gz).as_deref(), Some(&b"hello, world"[..]));
}

#[test]
fn rejects_a_truncated_trailer() {
    let gz = member(b"hello");
    for cut in 1..=8 {
        assert!(gunzip(&gz[..gz.len() - cut]).is_none(), "trailer short by {cut}");
    }
}

#[test]
fn rejects_a_crc_mismatch_in_any_member() {
    let bad = member_with(b"world", crc32(b"world") ^ 1, 5);
    assert!(gunzip(&bad).is_none(), "sole member");
    let mut gz = member(b"hello");
    gz.extend(bad);
    assert!(gunzip(&gz).is_none(), "second member");
}

#[test]
fn rejects_an_isize_mismatch() {
    assert!(gunzip(&member_with(b"hello", crc32(b"hello"), 6)).is_none());
}

#[test]
fn stops_cleanly_at_trailing_garbage() {
    let mut gz = member(b"hello");
    gz.extend_from_slice(b"\0\0\0\0 padding");
    assert_eq!(gunzip(&gz).as_deref(), Some(&b"hello"[..]), "plain junk");
    let mut gz = member(b"hello");
    gz.extend_from_slice(&[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff, 7, 7, 7, 7, 7, 7, 7, 7]);
    assert_eq!(gunzip(&gz).as_deref(), Some(&b"hello"[..]), "junk shaped like a header");
}
