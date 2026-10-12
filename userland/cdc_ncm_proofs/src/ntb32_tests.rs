// NONOS Operating System (AGPL-3.0-or-later)
//! NTB32 received (NCM 1.0, 3.2.2 and 3.3.2, as recalled, unverified): an
//! NTH32 and NDP32s with 32-bit indices and lengths, checked as NTB16s are.

use crate::ntb::found;

const NTH32: u32 = 0x686D_636E; // "ncmh"
const NDP32: u32 = 0x306D_636E; // "ncm0"

/// A `len`-byte NTB32 whose NTH32 names its first NDP32 at `ndp`.
fn ntb32(len: usize, ndp: u32) -> Vec<u8> {
    let mut b = vec![0u8; len];
    b[0..4].copy_from_slice(&NTH32.to_le_bytes());
    b[4..6].copy_from_slice(&16u16.to_le_bytes());
    b[8..12].copy_from_slice(&(len as u32).to_le_bytes());
    b[12..16].copy_from_slice(&ndp.to_le_bytes());
    b
}

fn ndp32(b: &mut [u8], at: usize, sign: u32, next: u32, entries: &[(u32, u32)]) {
    let len = (16 + 8 * (entries.len() + 1)) as u16;
    b[at..at + 4].copy_from_slice(&sign.to_le_bytes());
    b[at + 4..at + 6].copy_from_slice(&len.to_le_bytes());
    b[at + 8..at + 12].copy_from_slice(&next.to_le_bytes());
    for (i, (index, l)) in entries.iter().enumerate() {
        let e = at + 16 + 8 * i;
        b[e..e + 4].copy_from_slice(&index.to_le_bytes());
        b[e + 4..e + 8].copy_from_slice(&l.to_le_bytes());
    }
}

#[test]
fn an_ntb32_gives_its_datagrams_across_its_chain() {
    let mut b = ntb32(1024, 16);
    ndp32(&mut b, 16, NDP32, 64, &[(200, 60), (300, 70)]);
    ndp32(&mut b, 64, NDP32, 0, &[(500, 100)]);
    assert_eq!(found(&b, 4096), [(200, 60), (300, 70), (500, 100)]);
}

#[test]
fn a_malformed_ntb32_gives_nothing_it_cannot_prove() {
    let mut b = ntb32(1024, 16);
    ndp32(&mut b, 16, NDP32, 16, &[(200, 60), (1000, 60), (300, 60)]);
    assert_eq!(found(&b, 4096), [(200, 60)], "past the end ends the NDP; a loop ends the chain");
    let mut c = ntb32(1024, 8);
    ndp32(&mut c, 16, NDP32, 0, &[(200, 60)]);
    assert!(found(&c, 4096).is_empty(), "an NDP inside the NTH32");
    let mut d = ntb32(1024, 16);
    ndp32(&mut d, 16, 0x316D_636E, 0, &[(200, 60)]);
    assert!(found(&d, 4096).is_empty(), "\"ncm1\": with CRCs");
    let e = ntb32(1024, 16);
    assert!(found(&e, 512).is_empty(), "a block longer than the input size");
}

#[test]
fn an_ntb32_is_capped_at_forty_datagrams() {
    let mut b = ntb32(4096, 16);
    let entries: Vec<(u32, u32)> = (0..60).map(|i| (1200 + i, 14)).collect();
    ndp32(&mut b, 16, NDP32, 0, &entries);
    assert_eq!(found(&b, 4096).len(), 40);
}
