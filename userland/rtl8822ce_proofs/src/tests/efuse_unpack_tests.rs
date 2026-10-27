// NONOS Operating System (AGPL-3.0-or-later)
//! The logical map is unpacked as rtw88 unpacks it: one- and two-byte
//! headers, words a header leaves out skipped, the first erased header the
//! end; and a block past the 768-byte map or the 388 packed bytes refuses
//! the whole map.

use super::fixtures::{blank, put};
use crate::efuse::deshuffle;

fn unpack(p: &[u8; 512]) -> (bool, [u8; 768]) {
    let mut log = [0xFF; 768];
    (deshuffle(p, &mut log), log)
}

#[test]
fn one_and_two_byte_headers_unpack() {
    let mut p = blank();
    let next = put(&mut p, 0, 0x1, 0x0, &[[1, 2], [3, 4], [5, 6], [7, 8]]);
    put(&mut p, next, 25, 0x0, &[[9, 10], [11, 12], [13, 14], [15, 16]]);
    let (ok, log) = unpack(&p);
    assert!(ok);
    assert_eq!(&log[8..16], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(&log[200..208], &[9, 10, 11, 12, 13, 14, 15, 16]);
}

#[test]
fn a_word_left_out_is_not_written_and_an_erased_header_ends() {
    let mut p = blank();
    let next = put(&mut p, 0, 0x2, 0b1010, &[[1, 2], [5, 6]]);
    p[next + 1] = 0x00; // after the 0xFF header, never read
    let (ok, log) = unpack(&p);
    assert!(ok);
    assert_eq!(&log[16..24], &[1, 2, 0xFF, 0xFF, 5, 6, 0xFF, 0xFF]);
}

#[test]
fn a_block_past_the_logical_map_refuses() {
    let mut p = blank();
    put(&mut p, 0, 96, 0x0, &[[1, 2], [3, 4], [5, 6], [7, 8]]);
    assert!(!unpack(&p).0);
    let mut p = blank();
    put(&mut p, 0, 95, 0x0, &[[1, 2], [3, 4], [5, 6], [7, 8]]);
    assert_eq!(&unpack(&p).1[760..768], &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn words_past_the_packed_area_refuse() {
    // Block 1 with every word, nine bytes a block: the 44th header is at 387,
    // the last packed byte, and its words would start at 388.
    let mut p = blank();
    p[..388].fill(0x10);
    assert!(!unpack(&p).0);
    // One block fewer ends at 386, and the erased byte at 387 ends the map.
    let mut p = blank();
    p[..387].fill(0x10);
    assert!(unpack(&p).0);
}
