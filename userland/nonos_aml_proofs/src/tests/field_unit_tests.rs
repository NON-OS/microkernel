// NONOS Operating System (AGPL-3.0-or-later)
//! Field units laid out from their FieldList (section 19.6.48): each named
//! field starts where the last element ended, under the access in force.

use crate::consts::{at, load};
use crate::enc::*;
use crate::namespace::{FieldUnit, Kind, NoteKind};

fn unit(aml: &crate::load::Aml, p: &str) -> FieldUnit {
    match aml.ns.get(at(aml, p).expect(p)).unwrap().kind {
        Kind::FieldUnit(u) => u,
        k => panic!("{p} is {k:?}"),
    }
}

#[test]
fn named_fields_follow_reserved_and_access_elements() {
    let list = cat(&[
        &[0x00, 0x08],
        b"FLD1",
        &[0x04],
        b"FLD2",
        &[0x04],
        &[0x01, 0x02, 0x00],
        b"FLD3",
        &[0x10],
        &[0x03, 0x04, 0x0B, 0x02],
        b"FLD4",
        &[0x08],
    ]);
    let region = cat(&[&op(0x5B80), &name("REG0"), &[0x01], &int(0x60), &int(8)]);
    let field = pkg(0x5B81, &cat(&[&name("REG0"), &[0x01], &list]));
    let (aml, _) = load(&scope("\\_SB", &cat(&[&region, &field])), &[]);
    let f1 = unit(&aml, "\\_SB.FLD1");
    assert_eq!((f1.bit_offset, f1.bit_len, f1.access), (8, 4, [0x01, 0, 0]));
    let f2 = unit(&aml, "\\_SB.FLD2");
    assert_eq!((f2.bit_offset, f2.bit_len), (12, 4));
    let f3 = unit(&aml, "\\_SB.FLD3");
    assert_eq!((f3.bit_offset, f3.bit_len, f3.access), (16, 16, [0x02, 0, 0]));
    let f4 = unit(&aml, "\\_SB.FLD4");
    assert_eq!((f4.bit_offset, f4.access), (32, [0x04, 0x0B, 0x02]));
    assert_eq!(f1.list, f4.list);
}

#[test]
fn index_and_bank_fields_make_units_and_connections_carry_over() {
    let ifield = pkg(0x5B86, &cat(&[&name("IDX0"), &name("DAT0"), &[0x01], b"IFL0", &[0x08]]));
    let conn = cat(&[&[0x02], &name("\\_SB.GPI0")]);
    let bank = pkg(
        0x5B87,
        &cat(&[&name("REG0"), &name("BNK0"), &int(1), &[0x01], &conn, b"BFL0", &[0x02]]),
    );
    let (aml, _) = load(&cat(&[&ifield, &bank]), &[]);
    assert_eq!(unit(&aml, "\\IFL0").bit_len, 8);
    let b = unit(&aml, "\\BFL0");
    assert!(b.connection.is_some());
    assert_eq!(b.bit_offset, 0);
}

#[test]
fn a_layout_past_four_gigabits_stops_with_a_note() {
    let huge = pkg_len(0x0FFF_FFFF - 4)[..].to_vec();
    let mut list = Vec::new();
    for i in 0..17u8 {
        list.extend_from_slice(&[0x00]);
        list.extend_from_slice(&huge);
        list.extend_from_slice(&[b'F', b'L', b'0' + i / 10, b'0' + i % 10, 0x01]);
    }
    let field = pkg(0x5B81, &cat(&[&name("REG0"), &[0x01], &list]));
    let (aml, _) = load(&field, &[]);
    assert_eq!(aml.notes.list.last().unwrap().kind, NoteKind::Layout);
    // The sixteenth field ends exactly at bit 2^32; the next cannot start.
    assert!(at(&aml, "\\FL15").is_some());
    assert!(at(&aml, "\\FL16").is_none());
}
