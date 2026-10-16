// NONOS Operating System (AGPL-3.0-or-later)
//! The flash window's edge, through the real map cutter: a window
//! straddling 0xFF00_0000 is refused a claim and mapped only below the
//! line, one inside it not at all, and one ending exactly at the line (as
//! the local APIC, I/O APIC, HPET and TPM windows all end below it) passes
//! whole.

use super::guard::{overlaps_flash, FLASH_WINDOW};
use crate::mmio_window::window::{window, WindowError};

const LINE: u64 = FLASH_WINDOW.0;

#[test]
fn a_window_straddling_the_line_is_refused_and_never_mapped_above_it() {
    let (base, size) = (LINE - 0x1000, 0x2000);
    assert!(overlaps_flash(base, size), "its claim is refused");
    let w = window(base, size, 0, size, &[FLASH_WINDOW]).expect("the page below");
    assert_eq!((w.page_start, w.page_bytes, w.usable), (base, 0x1000, 0x1000));
    assert_eq!(window(base, size, 0x1000, 0x1000, &[FLASH_WINDOW]), Err(WindowError::Protected));
}

#[test]
fn a_window_wholly_inside_is_refused_and_never_mapped() {
    let (base, size) = (0xFFF8_0000, 0x1000);
    assert!(overlaps_flash(base, size));
    assert_eq!(window(base, size, 0, size, &[FLASH_WINDOW]), Err(WindowError::Protected));
}

#[test]
fn a_window_ending_exactly_at_the_line_and_the_platform_windows_pass() {
    let edge = (LINE - 0x1000, 0x1000);
    let lapic = (0xFEE0_0000, 0x1000);
    let ioapic = (0xFEC0_0000, 0x1000);
    let hpet = (0xFED0_0000, 0x400);
    let tpm = (0xFED4_0000, 0x5000);
    for (base, size) in [edge, lapic, ioapic, hpet, tpm] {
        assert!(!overlaps_flash(base, size), "{base:#x} stays claimable");
        let w = window(base, size, 0, size, &[FLASH_WINDOW]).expect("mapped");
        assert_eq!(w.usable, size, "{base:#x} mapped whole");
    }
}
