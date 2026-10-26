// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's one Command write, put through the broker's own rule: the
//! masked form's merge (change.rs), the bits a driver may change
//! (command_bits.rs) and the Mmio and Dma gates (command_rule.rs). The
//! capsule holds Dma and Pio, no Mmio.

#[path = "../../../src/hardware/broker/pci/change.rs"]
#[allow(dead_code)]
mod change;
#[path = "../../../src/hardware/broker/pci/command_bits.rs"]
#[allow(dead_code)]
mod command_bits;
#[path = "../../../src/hardware/broker/pci/command_rule.rs"]
#[allow(dead_code)]
mod command_rule;

use crate::facts::pci::{COMMAND_BITS, COMMAND_MASK};

const IO: u16 = 1;
const MEM: u16 = 1 << 1;
const BM: u16 = 1 << 2;

/// What the broker writes for the driver's masked write over `current`.
fn taken(current: u16) -> Option<u16> {
    let merged = change::merge(COMMAND_BITS, COMMAND_MASK, current)?;
    let desired = command_bits::exact(merged, current)?;
    command_rule::command_value(desired, current, true, false).ok()
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
