// NONOS Operating System (AGPL-3.0-or-later)
//! PkgLength, section 20.2.4: every encoding size at its bounds, the reserved
//! bits, a length too short for itself, and a package past its parent.

use crate::enc::pkg_len;
use crate::error::ErrorKind;
use crate::grammar::{pkg_end, pkg_value, Cursor};

/// The PkgLength at the start of `bytes`, inside a parent `parent_end` long.
fn end_of(bytes: &[u8], parent_end: usize) -> Result<u32, ErrorKind> {
    let mut padded = bytes.to_vec();
    padded.resize(parent_end.max(bytes.len()), 0);
    let mut c = Cursor::new(&padded, 0, parent_end as u32);
    pkg_end(&mut c).map_err(|e| e.kind)
}

#[test]
fn every_size_round_trips_at_its_bounds() {
    for body in [0, 1, 61, 62, 63, 4092, 4093, 4094, 0xFFFFC, 0xFFFFD] {
        let enc = pkg_len(body);
        let total = body + enc.len();
        assert_eq!(end_of(&enc, total), Ok(total as u32), "body {body}");
    }
    // The largest a PkgLength can say, read as a value alone.
    let mut c = Cursor::new(&[0xCF, 0xFF, 0xFF, 0xFF], 0, 4);
    assert_eq!(pkg_value(&mut c).unwrap(), 0x0FFF_FFFF);
    assert_eq!(pkg_len(62).len(), 1);
    assert_eq!(pkg_len(63).len(), 2);
    assert_eq!(pkg_len(4093).len(), 2);
    assert_eq!(pkg_len(4094).len(), 3);
    assert_eq!(pkg_len(0xFFFFC).len(), 3);
    assert_eq!(pkg_len(0xFFFFD).len(), 4);
}

#[test]
fn reserved_bits_are_refused() {
    for lead in [0x50u8, 0x60, 0x90, 0xF0] {
        assert_eq!(end_of(&[lead, 0x10], 100), Err(ErrorKind::PkgLength), "{lead:#x}");
    }
}

#[test]
fn a_length_shorter_than_its_encoding_is_refused() {
    assert_eq!(end_of(&[0x00], 100), Err(ErrorKind::PkgLength));
    assert_eq!(end_of(&[0x41, 0x00], 100), Err(ErrorKind::PkgLength));
    assert_eq!(end_of(&[0x82, 0x00, 0x00], 100), Err(ErrorKind::PkgLength));
    assert_eq!(end_of(&[0x42, 0x00], 100), Ok(2));
}

#[test]
fn a_package_past_its_parent_is_refused() {
    assert_eq!(end_of(&[0x05], 4), Err(ErrorKind::PastParent));
    assert_eq!(end_of(&[0x05], 5), Ok(5));
}

#[test]
fn an_encoding_cut_short_is_truncated() {
    for cut in [&[0x40u8][..], &[0x80, 0x00], &[0xC0, 0x00, 0x00]] {
        assert_eq!(end_of(cut, cut.len()), Err(ErrorKind::Truncated));
    }
}
