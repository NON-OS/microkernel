// NONOS Operating System (AGPL-3.0-or-later)
//! rtw8822c_mac_init and rtw_drv_info_cfg written out again in rtw88's own
//! form (rtw8822c.c:2004-2131, mac.c:1364-1380), each access as the C makes
//! it, for the MAC table to be held to.

use crate::model::Chip;
use crate::regs::Mmio;

fn s8(c: &Chip, a: usize, b: u8) {
    c.write8(a, c.read8(a) | b);
}
fn c8(c: &Chip, a: usize, b: u8) {
    c.write8(a, c.read8(a) & !b);
}

pub fn rtw88(c: &Chip) {
    let v = c.read8(0x420) | (0x80 & !0x02 & !0x04);
    c.write8(0x420, v);
    c.write8(0x421, 0x1F);
    c.write16(0x428, 0x0A | 0x10 << 8);
    c.write32(0x514, 0x0A | 0x0E << 8 | 0x0A << 16 | 0x10 << 24);
    c.write16(0x63C, 0x0A | 0x0A << 8);
    c.write16(0x63E, 0x0E | 0x0E << 8);
    for (a, v) in [(0x430, 0x0100_0000), (0x434, 0x0807_0504), (0x43C, 0x0807_0504)] {
        c.write32(a, v);
    }
    for (a, v) in [(0x444, 0xFE01_F010u32), (0x448, 0x4000_0000), (0x44C, 0x003F_F010)] {
        c.write32(a, v);
    }
    for (a, v) in [(0x450, 0x4000_0000u32), (0x49C, 0x0600_F010), (0x4A0, 0x4000_03E0)] {
        c.write32(a, v);
    }
    c.write32(0x4A4, 0x0600_F015);
    c.write32(0x4A8, 0x0000_00E0);
    c.write8(0x455, 0x70);
    s8(c, 0x45E, 1 << 2);
    let pre: u16 = 0x1E0 | 1 << 11;
    c.write8(0x4E5, pre as u8);
    c.write8(0x4E6, (pre >> 8) as u8);
    c.write32(0x4C8, 0xFF | 0x08 << 8 | 0x3F << 16 | 0x3F << 24);
    c.write16(0x4CC + 2, 0x01 | 0x08 << 8);
    for a in [0x1448, 0x144A, 0x144C, 0x144E] {
        c.write8(a, 0x06);
    }
    c8(c, 0x426, 1 << 5);
    c.write32(0x440, c.read32(0x440) & !(0x3 << 21));
    for (a, v) in [(0x500, 0x002F_A226), (0x504, 0x005E_A328), (0x508, 0x005E_A42B)] {
        c.write32(a, v);
    }
    c.write32(0x50C, 0x0000_A44F);
    c.write8(0x512, 0x1C);
    c8(c, 0x521, ((1u16 << 12) >> 8) as u8);
    s8(c, 0x525, (((1u16 << 10) | (1 << 9) | (1 << 8)) >> 8) as u8);
    c.write32(0x24, c.read32(0x24) & !(0x3 << 20));
    c.write8(0x55C, 80);
    c.write8(0x638, 80);
    s8(c, 0x577, (1 << 3) | 0x3);
    c8(c, 0x5B4, 0x70);
    c.write16(0x522, 0);
    c.write8(0x51B, 0x09);
    c.write32(0x544, 0x05 | 0x1B << 16);
    c.write16(0x55E, 0x30 | 0x30 << 8);
    s8(c, 0x550, 1 << 3);
    c.write32(0x540, 0x04 | 0x064 << 8);
    c.write8(0x558, 0x04);
    c.write8(0x551, 0x10);
    c.write8(0x559, 0x02);
    c.write8(0x55D, 0xFF);
    super::mac_ref_wmac::rtw88_wmac(c);
}
