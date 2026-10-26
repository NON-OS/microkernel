// NONOS Operating System (AGPL-3.0-or-later)
//! The i2c-hid `_DSM` matcher (src/arch/x86_64/acpi/devices/i2c/dsm.rs) in
//! a seeded sweep. Undamaged, a `_DSM` under the HID-over-I2C GUID gives its
//! function-1 answer exactly when that answer is a register (1 to 0xFFFF),
//! and one under any other GUID gives nothing. Damaged anywhere, a touchpad
//! is still walked without a panic, and a register taken from `_DSM` is
//! never 0.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::devices::i2c::parse_hid_devices;
use crate::dsm_aml::Rng;
use crate::dsm_gen::{device, touchpad};

const SEEDS: u64 = 20_000;

/// One answer: the register the matcher took, if it took one from `_DSM`.
fn taken(aml: &[u8]) -> Option<Option<u16>> {
    let found = parse_hid_devices(aml);
    let d = found.first()?;
    Some(d.desc_reg_from_dsm.then_some(d.hid_desc_reg))
}

#[test]
fn the_dsm_register_is_its_function_one_constant_or_nothing() {
    let (mut kept, mut left) = (0, 0);
    for seed in 1..=SEEDS {
        let r = &mut Rng(seed);
        let v = [0, 1, 0x20, 0xFF, 0x100, 0xFFFF, 0x1_0000, r.below(1 << 32)][r.below(8) as usize];
        let right = r.below(6) != 0;
        let aml = device(&touchpad(r, v, right));
        let want = (right && (1..=0xFFFF).contains(&v)).then_some(v as u16);
        assert_eq!(taken(&aml), Some(want), "seed {seed}: {v:#x}, the GUID right: {right}");
        if want.is_some() {
            kept += 1
        } else {
            left += 1
        }
        let mut hostile: Vec<u8> = aml.clone();
        for _ in 0..1 + r.below(4) {
            let at = r.below(hostile.len() as u64) as usize;
            hostile[at] = r.below(256) as u8;
        }
        hostile.truncate(hostile.len() - r.below(hostile.len() as u64 / 2) as usize);
        if let Some(Some(reg)) = taken(&hostile) {
            assert_ne!(reg, 0, "seed {seed}: a register of 0 taken from a damaged _DSM");
        }
    }
    assert!(kept > SEEDS / 10 && left > SEEDS / 10, "both answers reached: {kept} {left}");
}
