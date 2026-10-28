/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Who owns the chip, the host driver or its firmware's low-power logic:
//! Linux v6.12 `mt792x_core.c`, `mt792xe_mcu_fw_pmctrl` and
//! `__mt792xe_mcu_drv_pmctrl`, and `mt7921/pci_mcu.c`,
//! `mt7921e_driver_own`. Each request is made up to ten times
//! (`MT792x_DRV_OWN_RETRY_COUNT`), each answer awaited 50 ms.

use crate::clock::delay_ms;
use crate::hw::addr::*;
use crate::hw::{poll, Regs};

const RETRIES: usize = 10;

/// Hand the chip to its firmware: `SET_OWN`, until `OWN_SYNC` reads set.
pub fn fw_own(regs: &Regs) -> bool {
    (0..RETRIES).any(|_| {
        regs.wr(CONN_ON_LPCTL, LPCR_HOST_SET_OWN);
        poll(regs, CONN_ON_LPCTL, LPCR_HOST_OWN_SYNC, LPCR_HOST_OWN_SYNC, 50)
    })
}

/// Take the chip for the driver: `CLR_OWN`, until `OWN_SYNC` reads clear.
/// Linux waits 2 to 3 ms after the write when ASPM is on; this waits 2 ms
/// always, since it does not ask the link's state first.
pub fn drv_own(regs: &Regs) -> bool {
    (0..RETRIES).any(|_| {
        regs.wr(CONN_ON_LPCTL, LPCR_HOST_CLR_OWN);
        delay_ms(2);
        poll(regs, CONN_ON_LPCTL, LPCR_HOST_OWN_SYNC, 0, 50)
    })
}

/// The MCU's own driver-ownership handshake before its firmware is
/// loaded: `DRV_OWN` into `MT_TOP_LPCR_HOST_BAND0`, until `FW_OWN` clears,
/// within 500 ms.
pub fn mcu_drv_own(regs: &Regs) -> bool {
    regs.wr(TOP_LPCR_HOST_BAND0, TOP_LPCR_HOST_DRV_OWN);
    poll(regs, TOP_LPCR_HOST_BAND0, TOP_LPCR_HOST_FW_OWN, 0, 500)
}
