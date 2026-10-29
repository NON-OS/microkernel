/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Bluetooth subsystem reset before a download, so the chip takes the
//! patch from its ROM's first state (Linux v6.12 `btmtk_usb_subsys_reset`).
//! MT7922: the reset register's bits 0x2020 set, the endpoint reset
//! option, then its bit 0, and 100 ms. MT7921: the endpoint reset option,
//! the watchdog status read, the reset register to 1, both UDMA interrupt
//! states cleared, 20 ms, the reset register back to 0. Either way the
//! reset is done when `BT_MISC` says so, looked at every 20 ms for 1 s,
//! and the chip must still give its id. Linux goes on after a reset that
//! never finishes; this driver stops there.

use super::chip::Chip;
use super::fail::Fail;
use super::port::Port;
use super::reg::*;

const LOOK_MS: u64 = 20;
const LOOKS: usize = 50;

pub fn reset(p: &mut impl Port, chip: Chip) -> Result<(), Fail> {
    match chip {
        Chip::Mt7922 => {
            let v = uhw_read(p, SUBSYS_RST)?;
            uhw_write(p, SUBSYS_RST, v | 0x2020)?;
            uhw_write(p, EP_RST_OPT, EP_RST_IN_OUT)?;
            let v = uhw_read(p, SUBSYS_RST)?;
            uhw_write(p, SUBSYS_RST, v | 1)?;
            p.wait(100);
        }
        Chip::Mt7961 => {
            uhw_write(p, EP_RST_OPT, EP_RST_IN_OUT)?;
            uhw_read(p, WDT_STATUS)?;
            uhw_write(p, SUBSYS_RST, 1)?;
            for r in [UDMA_INT_STA_BT, UDMA_INT_STA_BT1] {
                uhw_write(p, r, 0xff)?;
                uhw_read(p, r)?;
            }
            p.wait(20);
            uhw_write(p, SUBSYS_RST, 0)?;
            uhw_read(p, SUBSYS_RST)?;
        }
    }
    let mut done = false;
    for _ in 0..LOOKS {
        if uhw_read(p, BT_MISC)? & RST_DONE != 0 {
            done = true;
            break;
        }
        p.wait(LOOK_MS);
    }
    if chip == Chip::Mt7922 {
        uhw_write(p, UDMA_INT_STA_BT, 0xff)?;
    }
    match done && read(p, DEV_ID)? != 0 {
        true => Ok(()),
        false => Err(Fail::Reset),
    }
}
