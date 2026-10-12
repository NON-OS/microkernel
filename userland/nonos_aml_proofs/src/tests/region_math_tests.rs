// NONOS Operating System (AGPL-3.0-or-later)
//! The pure arithmetic of fields: access widths, the units a field touches,
//! UpdateRule merges, bit strings and the Global Lock's steps.

use crate::region::*;

#[test]
fn access_widths() {
    let io = Space::SYSTEM_IO;
    assert_eq!(access_width(1, 0, 32, io), 1, "ByteAcc");
    assert_eq!(access_width(3, 0, 8, io), 4, "DWordAcc");
    assert_eq!(access_width(0, 32, 32, io), 4, "AnyAcc, an aligned dword");
    assert_eq!(access_width(0, 16, 16, io), 2, "AnyAcc, an aligned word");
    assert_eq!(
        access_width(0, 8, 16, io),
        4,
        "AnyAcc, a word off its alignment: the dword holding it"
    );
    assert_eq!(access_width(0, 4, 8, io), 2, "AnyAcc, across a byte inside a word");
    assert_eq!(access_width(0, 60, 8, io), 1, "AnyAcc, across a qword: bytes");
    assert_eq!(access_width(4, 0, 64, Space::EMBEDDED_CONTROL), 1, "EC is bytes whatever is asked");
}

#[test]
fn units_cover_the_field_exactly() {
    let p: Vec<Unit> = plan(12, 8, 1).collect();
    assert_eq!(
        p,
        [Unit { byte: 1, shift: 4, bits: 4, at: 0 }, Unit { byte: 2, shift: 0, bits: 4, at: 4 }]
    );
    let p: Vec<Unit> = plan(0, 32, 4).collect();
    assert_eq!(p, [Unit { byte: 0, shift: 0, bits: 32, at: 0 }]);
    for (off, len, w) in [(0u32, 1u32, 1u32), (7, 130, 2), (31, 33, 4), (5, 200, 8)] {
        let total: u32 = plan(off, len, w).map(|u| u.bits).sum();
        assert_eq!(total, len, "{off} {len} {w}");
        assert!(plan(off, len, w).all(|u| u.byte % w as u64 == 0 && u.shift + u.bits <= w * 8));
    }
}

#[test]
fn update_rules_fill_the_bits_the_field_does_not_own() {
    assert_eq!(merge(0xAB, 0x5, 4, 4, UpdateRule::Preserve), 0x5B);
    assert_eq!(merge(0xAB, 0x5, 4, 4, UpdateRule::WriteAsOnes) & 0xFF, 0x5F);
    assert_eq!(merge(0xAB, 0x5, 4, 4, UpdateRule::WriteAsZeros), 0x50);
    assert_eq!(merge(0, u64::MAX, 0, 64, UpdateRule::WriteAsZeros), u64::MAX);
    assert_eq!(UpdateRule::from_flags(0x20), UpdateRule::WriteAsOnes);
    assert_eq!(UpdateRule::from_flags(0x40), UpdateRule::WriteAsZeros);
    assert_eq!(UpdateRule::from_flags(0x60), UpdateRule::Preserve);
}

#[test]
fn bit_strings() {
    let mut b = vec![0u8; 3];
    set_bits(&mut b, 4, 12, 0xABC);
    assert_eq!(b, [0xC0, 0xAB, 0x00]);
    assert_eq!(get_bits(&b, 4, 12), 0xABC);
    assert_eq!(get_bits(&b, 20, 8), 0, "bits past the end read as zero");
    set_bits(&mut b, 22, 8, 0xFF);
    assert_eq!(b[2], 0xC0, "bits past the end are dropped");
}

#[test]
fn global_lock_steps() {
    assert_eq!(acquire_step(0), (2, true));
    assert_eq!(acquire_step(2), (3, false), "owned by firmware: marked pending");
    assert_eq!(acquire_step(1), (2, true), "a stale pending bit is cleared");
    assert_eq!(release_step(2), (0, false));
    assert_eq!(release_step(3), (0, true), "firmware was waiting");
}
