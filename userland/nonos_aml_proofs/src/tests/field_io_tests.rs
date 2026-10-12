// NONOS Operating System (AGPL-3.0-or-later)
//! Field units read and written through a modelled region: widths, units,
//! values wider than an integer, bounds and handlers.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::rig::Rig;
use crate::run::raw;

fn io_rig(flags: u8, elements: &[Vec<u8>], body: &[u8]) -> Rig {
    let defs = cat(&[&region("REG0", 1, 0x60, 16), &field("REG0", flags, elements)]);
    Rig::new(&cat(&[&defs, &method("\\M", 1, body)])).connected()
}

#[test]
fn a_field_reads_what_was_written_through_its_units() {
    let els = [skip(4), unit("LOW", 12), unit("WORD", 16)];
    let body = cat(&[&store(&arg(0), &name("LOW")), &ret(&name("LOW"))]);
    let mut r = io_rig(0x01, &els, &body);
    assert_eq!(crate::run::num(&r.call("\\M", vec![Object::Integer(0xABC)]).unwrap()), 0xABC);
    assert_eq!(r.probe.bytes[&(1, 0x60)], 0xC0);
    assert_eq!(r.probe.bytes[&(1, 0x61)], 0xAB);
    // ByteAcc: every access a byte wide.
    assert!(r.probe.log.iter().all(|(_, _, w, _)| *w == 1));
}

#[test]
fn any_access_reads_an_aligned_register_in_one_piece() {
    let els = [skip(32), unit("REG", 32)];
    let mut r = io_rig(0x00, &els, &ret(&name("REG")));
    r.probe.bytes.extend([
        ((1, 0x64), 0x78),
        ((1, 0x65), 0x56),
        ((1, 0x66), 0x34),
        ((1, 0x67), 0x12),
    ]);
    assert_eq!(crate::run::num(&r.call("\\M", vec![Object::Integer(0)]).unwrap()), 0x1234_5678);
    assert_eq!(r.probe.log, [(1, 0x64, 4, None)]);
}

#[test]
fn a_field_wider_than_an_integer_is_a_buffer() {
    let body = cat(&[&store(&arg(0), &name("BIG")), &ret(&name("BIG"))]);
    let mut r = io_rig(0x01, &[unit("BIG", 72)], &body);
    let v = r.call("\\M", vec![Object::buffer((1..=9).collect())]).unwrap();
    assert_eq!(raw(&v), (1..=9).collect::<Vec<u8>>());
}

#[test]
fn bounds_handlers_and_refusals() {
    let past = [skip(120), unit("PAST", 16)];
    let mut r = io_rig(0x01, &past, &ret(&name("PAST")));
    assert_eq!(r.call("\\M", vec![Object::Integer(0)]).unwrap_err().kind, FaultKind::Index);
    let defs = cat(&[&region("REG0", 1, 0x60, 16), &field("REG0", 1, &[unit("ONE", 8)])]);
    let mut r = Rig::new(&cat(&[&defs, &method("\\M", 0, &ret(&name("ONE")))]));
    assert_eq!(r.call("\\M", vec![]).unwrap_err().kind, FaultKind::NoHandler);
    r.connect(1);
    assert!(r.call("\\M", vec![]).is_ok());
    r.probe.refuse = true;
    assert_eq!(r.call("\\M", vec![]).unwrap_err().kind, FaultKind::Region);
}

#[test]
fn an_embedded_controller_field_is_read_by_bytes() {
    let defs =
        cat(&[&region("ECR", 3, 0, 0x100), &field("ECR", 0x03, &[skip(0x80), unit("TMP", 32)])]);
    let mut r = Rig::new(&cat(&[&defs, &method("\\M", 0, &ret(&name("TMP")))])).connected();
    r.call("\\M", vec![]).unwrap();
    assert_eq!(
        r.probe.log.iter().map(|l| (l.1, l.2)).collect::<Vec<_>>(),
        [(0x10, 1), (0x11, 1), (0x12, 1), (0x13, 1)]
    );
}
