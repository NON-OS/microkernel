// NONOS Operating System (AGPL-3.0-or-later)
//! The definition block header, section 5.2.6.

use crate::block::Block;
use crate::enc::table;
use crate::error::ErrorKind;

#[test]
fn a_good_header_reads_back() {
    let t = table(b"SSDT", 1, &[0xA3]);
    let b = Block::new(&t, usize::MAX).unwrap();
    assert_eq!(b.signature(), *b"SSDT");
    assert_eq!(b.revision(), 1);
    assert!(!b.wide_integers(), "revision 1 tables use 32-bit integers");
    assert!(Block::new(&table(b"DSDT", 2, &[]), usize::MAX).unwrap().wide_integers());
    assert_eq!(b.oem_table_id(), *b"FIXTURES");
    assert_eq!(b.aml(), (36, 37));
}

#[test]
fn a_bad_checksum_is_refused() {
    let mut t = table(b"DSDT", 2, &[0xA3]);
    t[36] = 0xA5;
    assert_eq!(Block::new(&t, usize::MAX).unwrap_err().kind, ErrorKind::Checksum);
}

#[test]
fn a_length_field_that_disagrees_is_refused() {
    let mut t = table(b"DSDT", 2, &[0xA3]);
    t[4] = 35;
    assert_eq!(Block::new(&t, usize::MAX).unwrap_err().kind, ErrorKind::Header);
    t[4] = 38;
    assert_eq!(Block::new(&t, usize::MAX).unwrap_err().kind, ErrorKind::Header);
}

#[test]
fn trailing_bytes_past_the_length_field_are_not_read() {
    let mut t = table(b"DSDT", 2, &[0xA3]);
    t.extend_from_slice(&[0xEE; 8]);
    let b = Block::new(&t, usize::MAX).unwrap();
    assert_eq!(b.bytes.len(), 37);
}
