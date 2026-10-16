// NONOS Operating System (AGPL-3.0-or-later)
//! A finished bring-up given back whole: Bus Master off first, then the
//! line, the register window and the claim, in the reverse of the order
//! setup took them, as a failed attempt's grants are (grants_tests).

use nonos_libc::{given_back, EPOCH};

use crate::driver::Driver;
use crate::give_back::give_back;
use crate::regs::Regs;

#[test]
fn a_finished_bring_up_is_given_back_in_reverse_order_with_bus_master_off_first() {
    let mut window = vec![0u32; 0x100 / 4];
    // SAFETY: the vector's own 0x100 bytes, alive while `d` is used.
    let regs = unsafe { Regs::new(window.as_mut_ptr() as u64, 0x100) };
    let d = Driver {
        device_id: 5,
        pci_device: 0x51f0,
        claim_epoch: EPOCH,
        mmio_grant: 6,
        mmio_len: 0x100,
        irq_grant: 7,
        regs,
    };
    let before = given_back().len();
    give_back(&d);
    let order = [("pci_command", 2), ("irq_unbind", 7), ("mmio_unmap", 6), ("device_release", 5)];
    assert_eq!(given_back()[before..], order, "Memory Space alone (2) before any grant goes");
    give_back(&d);
    assert_eq!(given_back()[before + 4..], order, "a second give-back sends the same four and nothing else");
}
