// NONOS Operating System (AGPL-3.0-or-later)
//! What a TRANSFER payload may carry.

use crate::xfer::{parse, Kind};

#[test]
fn each_kind_carries_the_bytes_its_write_needs() {
    let x = parse(&[0x50, 1, 2, 7, 0]).expect("byte data read");
    assert_eq!((x.addr, x.read, x.kind, x.cmd, x.len), (0x50, true, Kind::ByteData, 7, 0));
    assert_eq!(parse(&[0x50, 0, 2, 7, 1, 0xEE]).map(|x| x.data[0]), Some(0xEE));
    assert_eq!(parse(&[0x50, 0, 3, 7, 2, 1, 2]).map(|x| (x.data[0], x.data[1])), Some((1, 2)));
    let block: Vec<u8> = [0x50, 0, 4, 7, 32].into_iter().chain(0..32).collect();
    assert_eq!(parse(&block).map(|x| x.data[31]), Some(31));
    assert_eq!(parse(&[0x0C, 0, 0, 0, 0]).map(|x| x.kind), Some(Kind::Quick));
    assert_eq!(parse(&[0x0C, 1, 1, 0, 0]).map(|x| x.kind), Some(Kind::Byte));
}

#[test]
fn lengths_that_do_not_match_the_kind_are_refused() {
    assert!(parse(&[0x50, 0, 2, 7, 0]).is_none(), "byte data write without its byte");
    assert!(parse(&[0x50, 0, 3, 7, 1, 1]).is_none(), "word data write with one byte");
    assert!(parse(&[0x50, 0, 4, 7, 0]).is_none(), "empty block write");
    let long: Vec<u8> = [0x50, 0, 4, 7, 33].into_iter().chain(0..33).collect();
    assert!(parse(&long).is_none(), "33-byte block");
    assert!(parse(&[0x50, 1, 2, 7, 1, 9]).is_none(), "a read carrying bytes");
    assert!(parse(&[0x50, 0, 2, 7, 1]).is_none(), "a length past the payload");
    assert!(parse(&[0x50, 1, 2, 7, 1]).is_none(), "a read naming a length");
    assert!(parse(&[0x50, 0, 2, 7, 0, 9]).is_none(), "a byte the length does not name");
}

#[test]
fn addresses_flags_and_kinds_outside_the_wire_are_refused() {
    assert!(parse(&[0x80, 1, 2, 0, 0]).is_none(), "an 8-bit address");
    assert!(parse(&[0x50, 3, 2, 0, 0]).is_none(), "an unknown flag");
    assert!(parse(&[0x50, 1, 5, 0, 0]).is_none(), "an unknown kind");
    assert!(parse(&[0x50, 1, 2, 0]).is_none(), "a short head");
}
