// NONOS Operating System (AGPL-3.0-or-later)
//! The architected timer's `clock-frequency`: taken as one cell when a board
//! names it, absent on QEMU virt and the Pi 4, whose firmware programs
//! CNTFRQ_EL0.

use crate::arch::fdt::find::timer;
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};
use crate::fixtures::{RPI4B, VIRT_GICV3};

const PPI: u32 = 1;

fn tree(rate: Option<&[u8]>) -> Vec<u8> {
    let mut b = Builder::default().begin("").prop("interrupt-parent", &cells(&[1]));
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b.begin("intc").prop("compatible", &text("arm,gic-400"));
    b = b.prop("interrupt-controller", &[]).prop("#interrupt-cells", &cells(&[3]));
    b = b.prop("phandle", &cells(&[1])).end();
    b = b.begin("timer").prop("compatible", &text("arm,armv8-timer"));
    let ints = [PPI, 13, 4, PPI, 14, 4, PPI, 11, 4, PPI, 10, 4];
    b = b.prop("interrupts", &cells(&ints));
    if let Some(rate) = rate {
        b = b.prop("clock-frequency", rate);
    }
    b.end().end().token(END).finish()
}

fn rate(blob: &[u8]) -> Option<u32> {
    timer::find(&Fdt::from_bytes(blob).unwrap()).unwrap().unwrap().clock_frequency
}

#[test]
fn a_board_whose_firmware_programs_cntfrq_names_no_rate() {
    assert_eq!(rate(VIRT_GICV3), None);
    assert_eq!(rate(RPI4B), None);
    assert_eq!(rate(&tree(None)), None);
}

#[test]
fn the_named_rate_is_one_cell_and_not_zero() {
    assert_eq!(rate(&tree(Some(&cells(&[54_000_000])))), Some(54_000_000));
    // The intids are read as before.
    let t = timer::find(&Fdt::from_bytes(&tree(Some(&cells(&[1])))).unwrap()).unwrap().unwrap();
    assert_eq!((t.nonsecure_phys_intid, t.virtual_intid), (30, 27));
    assert_eq!(rate(&tree(Some(&cells(&[0])))), None);
    assert_eq!(rate(&tree(Some(&cells(&[0, 54_000_000])))), None);
    assert_eq!(rate(&tree(Some(&[0x03, 0x37]))), None);
}
