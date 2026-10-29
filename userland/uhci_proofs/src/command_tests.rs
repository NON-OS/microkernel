// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's one Command write, put through the broker's own rule: the
//! masked form's split and merge (change.rs), the bits a driver may change
//! (command_bits.rs), the claim and the Mmio and Dma gates
//! (command_rule.rs). The capsule holds Dma and Pio, no Mmio.

#[path = "../../../src/hardware/broker/pci/change.rs"]
mod change;
#[path = "../../../src/hardware/broker/pci/command_bits.rs"]
mod command_bits;
#[path = "../../../src/hardware/broker/pci/command_rule.rs"]
mod command_rule;

use crate::facts::pci::{COMMAND_BITS, COMMAND_MASK};
use command_rule::{at_claim, command_value, Refused};

const IO: u16 = 1;
const MEM: u16 = 1 << 1;
const BM: u16 = 1 << 2;
const INTX: u16 = 1 << 10;
const COMMAND: u16 = 0x04;

/// What the broker writes for the driver's masked write over `current`.
fn taken(current: u16) -> Option<u16> {
    let merged = change::merge(COMMAND_BITS, COMMAND_MASK, current)?;
    let desired = command_bits::exact(merged, current)?;
    command_value(desired, current, true, false).ok()
}

#[test]
fn the_write_is_taken_over_what_the_boot_pass_leaves() {
    // The boot pass turns both decodes on and Bus Master off.
    for current in [IO | MEM, IO | MEM | BM, IO] {
        let now = taken(current).expect("taken");
        assert_eq!(now, IO | COMMAND_BITS, "from {current:#x}");
    }
}

#[test]
fn memory_space_is_named_and_bus_master_set() {
    assert_ne!(COMMAND_MASK & MEM, 0);
    assert_eq!(COMMAND_BITS & MEM, 0);
    assert_ne!(COMMAND_BITS & BM, 0);
}

#[test]
fn the_broker_reads_the_command_register_and_the_drivers_mask() {
    // nonos_libc's mk_pci_config_update puts the mask above the offset.
    let arg = u32::from(COMMAND) | (u32::from(COMMAND_MASK) << change::MASK_SHIFT);
    assert_eq!(change::split(arg), Some((COMMAND, COMMAND_MASK)));
}

#[test]
fn the_port_only_claim_then_the_write_leave_the_drivers_bits() {
    for firmware in [IO | MEM, IO | MEM | BM, IO, IO | MEM | BM | INTX] {
        let claimed = at_claim(firmware, true);
        assert_eq!(claimed & (MEM | BM), 0, "claimed from {firmware:#x}");
        assert_eq!(taken(claimed), Some(IO | COMMAND_BITS), "from {firmware:#x}");
    }
}

#[test]
fn the_plain_form_would_keep_memory_space_and_be_refused() {
    // Why the write names its bits: ORed on, Memory Space stays as left.
    let ored = command_bits::plain(COMMAND_BITS, IO | MEM).expect("ored");
    assert_ne!(ored & MEM, 0);
    assert_eq!(command_value(ored, IO | MEM, true, false), Err(Refused::MemorySpace));
}
