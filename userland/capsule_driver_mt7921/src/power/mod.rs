/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Power, ownership, identity and the subsystem reset, in the order
//! `mt7921_pci_probe` runs them.

pub mod own;
mod wake;

use crate::hw::addr::*;
use crate::hw::{poll, Regs};
use crate::ids::Chip;

/// The chip and its revision byte, as `mt7921_pci_probe` reads them
/// through the L1 window. `None` for a chip id this driver has no
/// firmware for (or a card that reads all ones).
pub fn identify(regs: &Regs) -> Option<(Chip, u8)> {
    let chip = Chip::from_ids(regs.rr(HW_CHIPID), regs.rr(HW_BOUND))?;
    Some((chip, (regs.rr(HW_REV) & 0xff) as u8))
}

/// The Wi-Fi subsystem reset (`mt792x_wfsys_reset`, the MT7921 family's
/// address): reset held 50 ms, released, then `INIT_DONE` within 500 ms.
pub fn wfsys_reset(regs: &Regs) -> bool {
    regs.clear(WFSYS_SW_RST, WFSYS_SW_RST_B);
    crate::clock::delay_ms(50);
    regs.set(WFSYS_SW_RST, WFSYS_SW_RST_B);
    poll(regs, WFSYS_SW_RST, WFSYS_SW_INIT_DONE, WFSYS_SW_INIT_DONE, 500)
}

/// The device's interrupts held off: the WFDMA's host enables and the
/// PCIe MAC's. Linux sets the MAC's to 0xff once it has an IRQ handler;
/// this driver polls and has none.
pub fn quiet(regs: &Regs) {
    regs.wr(HOST_INT_ENA, 0);
    regs.wr(PCIE_MAC_INT_ENABLE, 0);
}
