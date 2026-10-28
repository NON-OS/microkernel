/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The firmware: the package's two files for the chip, checked, then sent
//! (`mt7921e_mcu_init` and `mt792x_load_firmware` of Linux v6.12).

pub mod fetch;
mod load;
pub mod mode;
pub mod package;
pub mod patch;
mod piece;
pub mod pins;
pub mod ram;

pub use fetch::{fetch, Fetched};

use crate::dma::Dma;
use crate::hw::addr::*;
use crate::hw::{poll, Regs};
use crate::mcu::Mcu;

/// Where a download that had its files stopped; its number is in the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    BadPatch = 1,
    Semaphore = 2,
    Request = 3,
    Send = 4,
    Finish = 5,
    BadRam = 6,
    Start = 7,
    NotReady = 8,
    McuOwn = 9,
}

/// Download checked `patch` and `ram` bytes and wait for the firmware to
/// say it runs. Before it: the normal operation mode (`MT_SWDEF_MODE`,
/// `__mt7921_init_hardware`), the MCU's driver ownership and L0s off
/// (`mt7921e_mcu_init`). After: the firmware's ready bits within 1.5 s.
pub fn download(m: &mut Mcu, r: &Regs, d: &mut Dma, patch: &[u8], ram: &[u8]) -> Result<(), Step> {
    r.wr(SWDEF_MODE, 0);
    if !crate::power::own::mcu_drv_own(r) {
        return Err(Step::McuOwn);
    }
    r.rmw_field(PCIE_MAC_PM, PCIE_MAC_PM_L0S_DIS, 1);
    load::patch(m, r, d, patch)?;
    load::ram(m, r, d, ram)?;
    if !poll(r, CONN_ON_MISC, FW_N9_RDY, FW_N9_RDY, 1500) {
        return Err(Step::NotReady);
    }
    d.tx_fwdl.reclaim(r);
    Ok(())
}
