// NONOS Operating System (AGPL-3.0-or-later)
//! The register grant this driver asks for, put to the broker's own window
//! rule: every BAR size the PCI spec allows, page or not, is granted whole.

use crate::facts::pci::{register_grant, MMIO_MIN};
use crate::window::{window, WindowError};

#[test]
fn every_bar_size_from_the_registers_up_is_granted_whole() {
    for shift in 4..=20 {
        let bar = 1u64 << shift;
        if bar < MMIO_MIN {
            continue;
        }
        // A sub-page BAR may sit anywhere in its page, aligned to its size.
        for base in [0xFEB0_0000, 0xFEB0_0000 + bar.min(0x800)] {
            let w = window(base, bar, 0, register_grant(bar), &[]).expect("granted");
            assert_eq!(w.usable, bar, "{bar:#x} at {base:#x}");
        }
    }
}

#[test]
fn the_broker_refuses_a_grant_past_the_bar() {
    // What the driver asked for before: the BAR rounded up to a page.
    let bar = 0x100;
    assert_eq!(window(0xFEB0_0000, bar, 0, 0x1000, &[]), Err(WindowError::BadRange));
}

#[test]
fn an_msix_table_in_the_bars_page_refuses_the_grant_and_one_past_it_does_not() {
    // A sub-page BAR shares its page with whatever sits beside it; when that
    // is a function's MSI-X table, the grant must not map the page at all.
    let (base, bar) = (0xFEB0_0000u64, 0x100u64);
    let msix_same_page = [(0xFEB0_0800, 0xFEB0_0900)];
    let got = window(base, bar, 0, register_grant(bar), &msix_same_page);
    assert_eq!(got, Err(WindowError::Protected), "the MSI-X table's page is never mapped");
    let msix_next_page = [(0xFEB0_1000, 0xFEB0_1100)];
    let w = window(base, bar, 0, register_grant(bar), &msix_next_page).expect("granted");
    assert_eq!((w.page_start, w.page_bytes, w.usable), (base, 0x1000, bar));
}
