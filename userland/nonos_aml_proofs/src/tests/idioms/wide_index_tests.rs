// NONOS Operating System (AGPL-3.0-or-later)
//! An IndexField with DWordAcc and a unit at a byte offset inside a dword,
//! as the specification describes access width and UpdateRule (ACPI 6.5,
//! 19.6.64 and 19.6.48; recalled, unverified): the index register is
//! written with the byte offset of the aligned dword holding the unit, the
//! data register is touched one dword wide, and a write of part of that
//! dword fills the rest by the rule: Preserve reads it first; WriteAsOnes
//! and WriteAsZeros do not read, and fill with ones or zeros.

use crate::enc::*;
use crate::rig::Rig;

const IDX: u64 = 0x70;
const DAT: u64 = 0x74;

fn rig(rule: u8) -> Rig {
    let flags = 0x03 | (rule << 5);
    let index =
        pkg(0x5B86, &cat(&[&name("IDX"), &name("DAT"), &[flags], &skip(40), &unit("WIDE", 8)]));
    let defs = cat(&[
        &region("IOR", 1, IDX, 8),
        &field("IOR", 0x03, &[unit("IDX", 32), unit("DAT", 32)]),
        &index,
        &method("\\RW", 0, &ret(&name("WIDE"))),
        &method("\\WW", 0, &store(&int(0x5A), &name("WIDE"))),
    ]);
    let mut r = Rig::new(&defs).connected();
    for (i, b) in [0xCC, 0xDD, 0xEE, 0xFF].into_iter().enumerate() {
        r.probe.bytes.insert((1, DAT + i as u64), b);
    }
    r
}

#[test]
fn the_aligned_dword_through_the_pair() {
    let mut r = rig(0);
    assert_eq!(r.num("\\RW"), 0xDD, "byte 5 is byte 1 of the dword at 4");
    assert_eq!(r.probe.log, [(1, IDX, 4, Some(4)), (1, DAT, 4, None)]);
}

#[test]
fn a_partial_write_fills_the_dword_by_the_rule() {
    let rest = 0xFFEE_00CCu64;
    for (rule, read_first, written) in
        [(0, true, rest | 0x5A00), (1, false, 0xFFFF_5AFF), (2, false, 0x0000_5A00)]
    {
        let mut r = rig(rule);
        r.call("\\WW", vec![]).expect("the write runs");
        let mut want = vec![(1, IDX, 4, Some(4))];
        if read_first {
            want.extend([(1, DAT, 4, None), (1, IDX, 4, Some(4))]);
        }
        want.push((1, DAT, 4, Some(written)));
        assert_eq!(r.probe.log, want, "rule {rule}");
    }
}
