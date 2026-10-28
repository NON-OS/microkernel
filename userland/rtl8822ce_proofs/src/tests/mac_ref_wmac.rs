// NONOS Operating System (AGPL-3.0-or-later)
//! The WMAC, low-power and interrupt-migration part of rtw8822c_mac_init
//! and the rtw_drv_info_cfg tail, in rtw88's own form, including its
//! single-access read-modify-writes.

use crate::model::Chip;
use crate::regs::Mmio;

pub fn rtw88_wmac(c: &Chip) {
    c.write32(0x620, 0xFFFF_FFFF);
    c.write32(0x624, 0xFFFF_FFFF);
    c.write8(0x6DC + 2, 0x84);
    c.write8(0x640, 0x21);
    c.write8(0x639, 0x6A);
    c.write16(0x642, 0x40);
    c.write8(0x650 + 2, 0xC8);
    c.write8(0x66C + 2, 0x05);
    c.write32(0x6A0, 0xFFFF_FFFF);
    c.write16(0x6A4, 0xFFFF);
    c.write32(0x608, 0xE400_220E);
    c.write8(0x60C, (12288u32 >> 9) as u8);
    c.write8(0x604 + 2, 0x30);
    c.write8(0x604 + 1, 0x30);
    c.write32(0x1664, c.read32(0x1664) | 1 << 9);
    c.write32(0x7D0 + 8, 0xB081_0041);
    c.write8(0x7D4, 0x98);
    // init low power
    let fifothr = (1u32 << 26 | 1 << 24 | 1 << 22 | 1 << 20) >> 16;
    let v = (c.read16(0x1610 + 2) & 0xF00F) | fifothr as u16;
    c.write16(0x1610 + 2, v);
    let mut v: u16 = 1 << 13; // BIT_SET_RXPSF_PKTLENTHR(0, 1)
    v |= 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7;
    c.write16(0x1610, v);
    c.write32(0x1614, 0xFFFF_FFFF);
    let v = c.read16(0x1610) & !(1 << 5 | 1 << 6 | 1 << 4);
    c.write16(0x1610, (v & !0x7) | 0x07);
    c.write8(0x718, c.read8(0x718) | 1 << 6);
    c.write32(0x304, 0x3333_0000);
    // rtw_drv_info_cfg
    c.write8(0x60F, 4);
    let v = (c.read8(0x114 + 1) & 0xF0) | 0xF;
    c.write8(0x114 + 1, v);
    c.write32(0x608, c.read32(0x608) | 1 << 28);
    c.write32(0x7D0 + 4, c.read32(0x7D0 + 4) & !(1 << 8 | 1 << 9));
}
