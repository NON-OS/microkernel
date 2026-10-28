// NONOS Operating System (AGPL-3.0-or-later)

//! Start-up on a freshly powered MAC: the PCIe link held awake, the 8822C's
//! reference-clock delay off (rtw88 `rtw_pci_link_cfg`), the board facts
//! read, and the front end's PCIe setting (`rtw_pci_phy_cfg`'s tail).

use crate::setup::Mapped;
use crate::{efuse, pcie, status};

pub fn link_and_board(mapped: &mut Mapped) {
    // As the RTL8821CE: an L1 link gates the chip's clocks, and a gated block
    // answers reads with zeros, so the link is held out of L1 first.
    match pcie::hold_link_awake(&mapped.regs) {
        Ok(()) => {}
        Err(pcie::DbiTimeout::Read) => {
            status::line(b"[rtl8822ce] pcie link config: dbi read timed out\n")
        }
        Err(pcie::DbiTimeout::Write) => {
            status::line(b"[rtl8822ce] pcie link config: dbi write timed out\n")
        }
    }
    if !pcie::clock_delay_off(&mapped.regs) {
        status::line(b"[rtl8822ce] pcie clock delay: dbi write timed out\n");
    }
    // The efuse is read here, on a freshly powered MAC, where rtw88 reads it.
    mapped.efuse = efuse::read(&mapped.regs);
    match mapped.efuse {
        Some(info) => pcie::board(&mapped.regs, info.rfe),
        None => status::line(b"[rtl8822ce] efuse unreadable on a freshly powered mac\n"),
    }
}
