// NONOS Operating System (AGPL-3.0-or-later)
//! The window parse, against coreboot gpio.asl, Broxton and AMD templates.

use alloc::vec::Vec;

use super::super::crs::parse_gpio_crs;
use crate::arch::x86_64::acpi::aml::types::GpioController;
use crate::fixtures::{ext_interrupt, memory32_fixed};

/// A `_CRS` Buffer with a two-byte PkgLength, room for five windows.
fn template(desc: &[u8]) -> Vec<u8> {
    let mut inner = desc.to_vec();
    inner.extend_from_slice(&[0x79, 0x00]);
    let pkg = 2 + 3 + inner.len();
    let mut out = alloc::vec![0x11, 0x40 | (pkg & 0xF) as u8, (pkg >> 4) as u8, 0x0B];
    out.extend_from_slice(&(inner.len() as u16).to_le_bytes());
    out.extend_from_slice(&inner);
    out
}

fn parse(hid: &[u8; 8], desc: &[u8]) -> GpioController {
    let mut ctl = GpioController::new(*hid);
    parse_gpio_crs(&template(desc), &mut ctl);
    ctl
}

#[test]
fn run_time_windows_leave_no_static_window() {
    // coreboot tigerlake gpio.asl: four Memory32Fixed (ReadWrite, 0, 0).
    let mut d = Vec::new();
    for _ in 0..4 {
        d.extend_from_slice(&memory32_fixed(0, 0));
    }
    d.extend_from_slice(&ext_interrupt(14));
    assert_eq!(parse(b"INT34C5\0", &d).window_count, 0);
    // A static window followed by a patched one must not leave a lone,
    // misnumbered first community.
    let mut d = memory32_fixed(0xFD6E_0000, 0x1_0000);
    d.extend_from_slice(&memory32_fixed(0, 0));
    assert_eq!(parse(b"INT34C5\0", &d).window_count, 0);
}

#[test]
fn static_windows_are_kept_in_order() {
    let mut d = ext_interrupt(7);
    d.extend_from_slice(&memory32_fixed(0xFED8_1500, 0x400));
    let amd = parse(b"AMDI0030", &d);
    assert_eq!((amd.window_count, amd.windows[0]), (1, (0xFED8_1500, 0x400)));
    let mut d = Vec::new();
    for i in 0..6u32 {
        d.extend_from_slice(&memory32_fixed(0xD0C0_0000 + i * 0x1_0000, 0x4000));
    }
    let many = parse(b"INT3452\0", &d);
    assert_eq!(many.window_count, 5, "the record holds five windows");
    assert_eq!(many.windows[4], (0xD0C4_0000, 0x4000));
}
