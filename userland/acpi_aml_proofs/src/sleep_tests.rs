// NONOS Operating System (AGPL-3.0-or-later)
//! S5 entry writes the registers ACPICA's hwsleep.c writes, in its order.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::hw::fadt_decode::*;
use crate::arch::x86_64::acpi::hw::gas::Gas;
use crate::arch::x86_64::acpi::hw::sleep::*;
use crate::bus_fake::{FakeBus, Op};
use crate::fadt_tests::{gemini_lake_fadt, FadtBuilder};

const S5_INTEL: SleepTypes = SleepTypes { a: 7, b: 7 };

#[test]
fn gemini_lake_s5_clears_status_quiesces_gpes_then_writes_slp_typ_and_slp_en() {
    let fadt = decode_fadt(&gemini_lake_fadt()).unwrap();
    let mut bus = FakeBus::default();
    // SCI_EN set, plus a stale SLP_EN and the write-only GBL_RLS.
    bus.io.insert(0x404, 0x2005);
    let mut flushed = false;
    let r = enter_sleep(&mut bus, &fadt, S5_INTEL, || flushed = true);
    assert_eq!(r, Ok(()));

    let w = bus.writes();
    assert_eq!(w[0], Op::IoWrite(0x400, 16, 0x8731), "all fixed status, WAK_STS included");
    // 0x20 GPE block bytes: 16 enable bytes zeroed, then 16 status bytes cleared.
    for i in 0..16u16 {
        assert_eq!(w[1 + i as usize], Op::IoWrite(0x430 + i, 8, 0x00));
        assert_eq!(w[17 + i as usize], Op::IoWrite(0x420 + i, 8, 0xFF));
    }
    assert_eq!(w[33], Op::IoWrite(0x404, 16, 0x1C01), "SLP_TYP=7 alone, SCI_EN kept");
    assert_eq!(w[34], Op::IoWrite(0x404, 16, 0x3C01), "then SLP_EN");
    assert_eq!(w.len(), 35);
    assert!(flushed, "caches are flushed before SLP_EN");
}

#[test]
fn both_pm1_control_blocks_get_their_own_slp_typ() {
    let t = FadtBuilder::new(244)
        .u32(off::PM1A_CNT_BLK, 0x404)
        .u32(off::PM1B_CNT_BLK, 0x504)
        .u8(off::PM1_CNT_LEN, 2)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    enter_sleep(&mut bus, &fadt, SleepTypes { a: 5, b: 3 }, || {}).unwrap();
    let w: Vec<Op> = bus.writes();
    assert_eq!(
        w,
        [
            Op::IoWrite(0x404, 16, 5 << 10),
            Op::IoWrite(0x504, 16, 3 << 10),
            Op::IoWrite(0x404, 16, (5 << 10) | (1 << 13)),
            Op::IoWrite(0x504, 16, (3 << 10) | (1 << 13)),
        ]
    );
}

#[test]
fn qemu_q35_s5_is_slp_typ_zero_on_port_0x604() {
    let t = FadtBuilder::new(244)
        .u32(off::PM1A_EVT_BLK, 0x600)
        .u32(off::PM1A_CNT_BLK, 0x604)
        .u8(off::PM1_EVT_LEN, 4)
        .u8(off::PM1_CNT_LEN, 2)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    bus.io.insert(0x604, 1);
    enter_sleep(&mut bus, &fadt, SleepTypes { a: 0, b: 0 }, || {}).unwrap();
    assert_eq!(bus.writes().last(), Some(&Op::IoWrite(0x604, 16, 0x2001)));
}

#[test]
fn hardware_reduced_platforms_use_the_sleep_control_register() {
    let t = FadtBuilder::new(276)
        .u32(off::FLAGS, FLAG_HW_REDUCED_ACPI)
        .gas(off::SLEEP_CONTROL_REG, Gas::io(0x1008, 1))
        .gas(off::SLEEP_STATUS_REG, Gas::io(0x100C, 1))
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    enter_sleep(&mut bus, &fadt, SleepTypes { a: 5, b: 0 }, || {}).unwrap();
    assert_eq!(
        bus.writes(),
        [Op::IoWrite(0x100C, 8, 0x80), Op::IoWrite(0x1008, 8, ((5 << 2) & 0x1C) | 0x20)]
    );
}

#[test]
fn no_control_register_is_refused_before_any_write() {
    let fadt = decode_fadt(&FadtBuilder::new(244).sealed()).unwrap();
    let mut bus = FakeBus::default();
    assert_eq!(enter_sleep(&mut bus, &fadt, S5_INTEL, || {}), Err(SleepError::NoControlRegister));
    assert!(bus.ops.is_empty());
    let reduced =
        decode_fadt(&FadtBuilder::new(276).u32(off::FLAGS, FLAG_HW_REDUCED_ACPI).sealed()).unwrap();
    assert_eq!(
        enter_sleep(&mut bus, &reduced, S5_INTEL, || {}),
        Err(SleepError::NoControlRegister)
    );
    assert!(bus.ops.is_empty());
}

#[test]
fn pm1_control_values_keep_every_bit_but_the_sleep_fields() {
    assert_eq!(pm1_control_values(0xFFFF, S5_INTEL), (0xDFFB, 0xDFFB));
    assert_eq!(pm1_control_values(0, SleepTypes { a: 0xFF, b: 0 }), (7 << 10, 0));
}

#[test]
fn pm1_event_blocks_split_into_status_and_enable_halves() {
    let (s, e) = pm1_halves(&Gas::io(0x400, 4));
    assert_eq!((s.address, s.bit_width), (0x400, 16));
    assert_eq!((e.address, e.bit_width), (0x402, 16));
    assert_eq!(pm1_halves(&Gas::empty()), (Gas::empty(), Gas::empty()));
}
