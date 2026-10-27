// NONOS Operating System (AGPL-3.0-or-later)

//! `__rtw8852bx_power_trim` (Linux v6.12 rtw89 `rtw8852b_common.c`): the
//! factory's thermal-meter and PA bias trims from the phycap map, each
//! written only when programmed (not both 0xff).

use super::bb::Bb;
use crate::regs::Mmio;

const RR_TM2: u32 = 0x43;
const RR_TM2_OFF: u32 = 0xf_0000;
const RR_BIASA: u32 = 0x60;
const RR_BIASA_TXG: u32 = 0xf000;
const RR_BIASA_TXA: u32 = 0xf_0000;

/// `__thm_setting`: bit 0 moves to bit 3, bits 4:1 down to 3:0.
pub const fn thermal_setting(raw: u8) -> u32 {
    (((raw & 0x1) << 3) | ((raw & 0x1f) >> 1)) as u32
}

pub fn power_trim(r: &impl Mmio, thermal: Option<[u8; 2]>, pa_bias: Option<[u8; 2]>) {
    let bb = Bb(r);
    if let Some(t) = thermal {
        for (path, &raw) in t.iter().enumerate() {
            bb.write_rf(path as u8, RR_TM2, RR_TM2_OFF, thermal_setting(raw));
        }
    }
    if let Some(b) = pa_bias {
        for (path, &raw) in b.iter().enumerate() {
            bb.write_rf(path as u8, RR_BIASA, RR_BIASA_TXG, (raw & 0xf) as u32);
            bb.write_rf(path as u8, RR_BIASA, RR_BIASA_TXA, (raw >> 4) as u32);
        }
    }
}
