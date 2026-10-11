// NONOS Operating System (AGPL-3.0-or-later)
//! FieldList, section 20.2.5.2: each element kind, and malformed ones.

use crate::arena::Imm;
use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::parse;
use crate::op::Op;

fn field(list: &[u8]) -> Vec<u8> {
    pkg(0x5B81, &cat(&[&name("REG0"), &[0x01], list]))
}

#[test]
fn every_element_kind_parses() {
    let list = cat(&[
        &[0x00, 0x08],
        b"STS0",
        &[0x01],
        &[0x01, 0x03, 0x00],
        &[0x03, 0x04, 0x0B, 0x10],
        &[0x02],
        &name("\\_SB.GPI0"),
        &[0x02],
        &buffer(2, &[0x79, 0x00]),
        b"WIDE",
        &pkg_len(0x123)[..],
    ]);
    let (arena, root) = parse(&field(&list)).unwrap();
    let f = arena.kids(root)[0];
    let fl = arena.kids(f)[2];
    let got: Vec<(Op, Imm)> =
        arena.kids(fl).iter().map(|k| arena.node(*k).unwrap()).map(|n| (n.op, n.imm)).collect();
    assert_eq!(got[0], (Op::RESERVED_FIELD, Imm::Int(8)));
    assert_eq!(got[1], (Op::NAMED_FIELD, Imm::Int(1)));
    assert_eq!(got[2], (Op::ACCESS_FIELD, Imm::Int(0x03)));
    assert_eq!(got[3], (Op::EXTENDED_ACCESS_FIELD, Imm::Int(0x10_0B04)));
    assert_eq!((got[4].0, got[5].0), (Op::CONNECT_FIELD, Op::CONNECT_FIELD));
    assert_eq!(got[6].0, Op::NAMED_FIELD);
    assert_eq!(arena.node(arena.kids(fl)[1]).unwrap().at, 36 + 2 + 1 + 4 + 1 + 2);
}

#[test]
fn malformed_elements_are_refused() {
    assert_eq!(parse(&field(b"st00\x08")).unwrap_err().kind, ErrorKind::Opcode(b's' as u16));
    assert_eq!(parse(&field(b"S-00\x08")).unwrap_err().kind, ErrorKind::Name);
    assert_eq!(parse(&field(&[0x04, 0x00])).unwrap_err().kind, ErrorKind::Opcode(4));
    assert_eq!(parse(&field(&[0x01, 0x01])).unwrap_err().kind, ErrorKind::Truncated);
}
