// NONOS Operating System (AGPL-3.0-or-later)
//! UpdateRule on a partial write, IndexField and BankField.

use crate::enc::*;
use crate::object::Object;
use crate::rig::Rig;

fn partial(flags: u8) -> Rig {
    let defs =
        cat(&[&region("REG0", 1, 0x60, 4), &field("REG0", flags, &[skip(2), unit("MID", 3)])]);
    let mut r =
        Rig::new(&cat(&[&defs, &method("\\M", 0, &store(&int(0b101), &name("MID")))])).connected();
    r.probe.bytes.insert((1, 0x60), 0b1100_0010);
    r.call("\\M", vec![]).unwrap();
    r
}

#[test]
fn preserve_reads_first_and_keeps_the_other_bits() {
    let r = partial(0x01);
    assert_eq!(r.probe.log, [(1, 0x60, 1, None), (1, 0x60, 1, Some(0b1101_0110))]);
}

#[test]
fn write_as_ones_and_zeros_do_not_read() {
    let r = partial(0x21);
    assert_eq!(r.probe.log, [(1, 0x60, 1, Some(0b1111_0111))]);
    let r = partial(0x41);
    assert_eq!(r.probe.log, [(1, 0x60, 1, Some(0b0001_0100))]);
}

#[test]
fn an_index_field_writes_the_index_then_moves_the_data() {
    let regs = field("REG0", 0x01, &[unit("IDX", 8), unit("DAT", 8)]);
    let ifield =
        pkg(0x5B86, &cat(&[&name("IDX"), &name("DAT"), &[0x01], &skip(0x10), &unit("CFG", 8)]));
    let body = cat(&[&store(&int(0x5A), &name("CFG")), &ret(&name("CFG"))]);
    let src = cat(&[&region("REG0", 1, 0x70, 2), &regs, &ifield, &method("\\M", 0, &body)]);
    let mut r = Rig::new(&src).connected();
    assert_eq!(r.num("\\M"), 0x5A);
    assert_eq!(r.probe.log[..2], [(1, 0x70, 1, Some(2)), (1, 0x71, 1, Some(0x5A))]);
    assert_eq!(r.probe.log[2..], [(1, 0x70, 1, Some(2)), (1, 0x71, 1, None)]);
}

#[test]
fn a_bank_field_selects_its_bank_before_each_access() {
    let sel = cat(&[&region("SEL0", 1, 0x80, 1), &field("SEL0", 0x01, &[unit("BNK", 8)])]);
    let bank = pkg(0x5B87, &cat(&[&name("REG0"), &name("BNK"), &int(3), &[0x01], &unit("BV", 8)]));
    let src = cat(&[
        &region("REG0", 1, 0x90, 1),
        &sel,
        &bank,
        &method("\\M", 1, &store(&arg(0), &name("BV"))),
    ]);
    let mut r = Rig::new(&src).connected();
    r.call("\\M", vec![Object::Integer(7)]).unwrap();
    assert_eq!(r.probe.log, [(1, 0x80, 1, Some(3)), (1, 0x90, 1, Some(7))]);
}
