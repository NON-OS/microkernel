// NONOS Operating System (AGPL-3.0-or-later)
//! A _CRS that patches a resource template, as the specification's own
//! examples do (ACPI 6.5, 19.6.20 to 19.6.24 for the Create*Field
//! operators, 6.4 for the descriptors; recalled, unverified): fields made
//! over the named template change the named buffer; fields made over a
//! Local's copy change only the copy (19.3.5.8: Store to a Local copies).

use crate::enc::*;
use crate::object::Object;
use crate::rig::Rig;
use crate::run::raw as bytes;

/// An IRQ descriptor (0x22, mask word) listing IRQ 11 by default, and a
/// 32-bit fixed memory range (0x86, length 9, write bit, base, length),
/// with the end tag.
fn template() -> Vec<u8> {
    let irq = [0x22, 0x00, 0x08];
    let mem = [0x86, 0x09, 0x00, 0x01, 0, 0, 0, 0, 0, 0x10, 0, 0];
    cat(&[&irq, &mem, &[0x79, 0x00]])
}

fn rig() -> Rig {
    let bytes = template();
    let fields = cat(&[
        &e(0x8B, &[&name("\\RBUF"), &int(1), &name("IRQM")]),
        &e(0x8A, &[&name("\\RBUF"), &int(7), &name("BAS0")]),
        &e(0x8C, &[&name("\\RBUF"), &int(3), &name("TAG0")]),
        &e(0x8D, &[&name("\\RBUF"), &int(8 * 6), &name("RW00")]),
        &e(0x8D, &[&name("\\RBUF"), &int(8), &name("IRQ0")]),
    ]);
    let crs = cat(&[
        &fields,
        &store(&e(0x79, &[&int(1), &arg(0), &[0]]), &name("IRQM")),
        &store(&arg(1), &name("BAS0")),
        &store(&int(0), &name("RW00")),
        &store(&int(1), &name("IRQ0")),
        &ret(&name("\\RBUF")),
    ]);
    let copy = cat(&[
        &store(&name("\\RBUF"), &local(0)),
        &e(0x8F, &[&local(0), &int(3), &name("QW00")]),
        &store(&arg(0), &name("QW00")),
        &ret(&local(0)),
    ]);
    let defs = cat(&[
        &name_obj("\\RBUF", &buffer(bytes.len() as u64, &bytes)),
        &method("\\CRS", 2, &crs),
        &method("\\CPY", 1, &copy),
        &method(
            "\\TAG",
            0,
            &cat(&[&e(0x8C, &[&name("\\RBUF"), &int(3), &name("T0")]), &ret(&name("T0"))]),
        ),
    ]);
    Rig::new(&defs).connected()
}

#[test]
fn the_named_template_is_patched_in_place() {
    let mut r = rig();
    let got =
        bytes(&r.call("\\CRS", vec![Object::Integer(4), Object::Integer(0xFED0_0000)]).unwrap());
    let mut want = template();
    want[1..3].copy_from_slice(&(1u16 << 4 | 1).to_le_bytes());
    want[7..11].copy_from_slice(&0xFED0_0000u32.to_le_bytes());
    want[6] = 0x00;
    assert_eq!(got, want, "IRQs 4 and 0 in one mask byte, the base, the write bit cleared");
    assert_eq!(bytes(&r.call("\\RBUF", vec![]).unwrap()), want, "the name holds the patch");
    assert_eq!(r.num("\\TAG"), 0x86, "bytes beside the fields untouched");
    assert_eq!(bytes(&r.call("\\CPY", vec![Object::Integer(u64::MAX)]).unwrap())[3..11], [0xFF; 8]);
    assert_eq!(bytes(&r.call("\\RBUF", vec![]).unwrap()), want, "a Local's copy leaves it be");
}
