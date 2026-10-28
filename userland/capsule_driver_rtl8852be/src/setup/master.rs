// NONOS Operating System (AGPL-3.0-or-later)

//! The function's PCI command register: Memory Space from the claim, Bus
//! Master once the power cycle showed the MAC off; and the release of a chip
//! that must not be served, Bus Master off before the claim goes.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// Write the command register: Memory Space always, Bus Master when
/// `master`. The broker's answer: 0, or a negative errno.
pub fn command_rc(device_id: u64, epoch: u64, master: bool) -> i64 {
    let bus = if master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    mk_pci_config_write(device_id, epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_MEMORY_SPACE | bus)
}

/// `command_rc`, as whether the broker took the write.
pub fn command(device_id: u64, epoch: u64, master: bool) -> bool {
    command_rc(device_id, epoch, master) >= 0
}

/// Bus Master off, then the claim given back, which takes the register
/// window and every DMA region mapped under it.
pub fn release(device_id: u64, epoch: u64) {
    let _ = command(device_id, epoch, false);
    let _ = mk_device_release(device_id);
}
