/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The chip's registers over the default endpoint, as Linux v6.12
//! `btmtk.c` reaches them: the chip's own read (vendor request 0x63, type
//! 0xC0) and the USB wrapper's read (0x01, type 0xDE) and write (0x02,
//! type 0x5E). The register's high half is wValue, its low half wIndex,
//! and the value four octets little-endian.

use nonos_bt_hci::usb::Setup;

use super::fail::Fail;
use super::port::Port;

pub const DEV_ID: u32 = 0x7001_0200;
pub const FW_VERSION: u32 = 0x8002_1004;
pub const FW_FLAVOR: u32 = 0x7001_0020;
pub const BT_MISC: u32 = 0x7000_2510;
pub const SUBSYS_RST: u32 = 0x7000_2610;
pub const UDMA_INT_STA_BT: u32 = 0x7400_0024;
pub const UDMA_INT_STA_BT1: u32 = 0x7400_0308;
pub const WDT_STATUS: u32 = 0x7400_03a0;
/// The device endpoint reset option, and its value for both directions.
pub const EP_RST_OPT: u32 = 0x7401_1890;
pub const EP_RST_IN_OUT: u32 = 0x0001_0001;
/// `BT_MISC`'s bit for a subsystem reset done.
pub const RST_DONE: u32 = 1 << 8;

fn setup(request_type: u8, request: u8, reg: u32) -> Setup {
    Setup { request_type, request, value: (reg >> 16) as u16, index: reg as u16 }
}

fn read4(p: &mut impl Port, s: Setup) -> Result<u32, Fail> {
    let mut b = [0u8; 4];
    match p.control_in(s, &mut b) {
        Ok(4) => Ok(u32::from_le_bytes(b)),
        _ => Err(Fail::Usb),
    }
}

pub fn read(p: &mut impl Port, reg: u32) -> Result<u32, Fail> {
    read4(p, setup(0xc0, 0x63, reg))
}

pub fn uhw_read(p: &mut impl Port, reg: u32) -> Result<u32, Fail> {
    read4(p, setup(0xde, 0x01, reg))
}

pub fn uhw_write(p: &mut impl Port, reg: u32, val: u32) -> Result<(), Fail> {
    p.control_out(setup(0x5e, 0x02, reg), &val.to_le_bytes()).map_err(|_| Fail::Usb)
}
