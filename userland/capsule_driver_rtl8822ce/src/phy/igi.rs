// NONOS Operating System (AGPL-3.0-or-later)

//! rtw88 `rtw8822c_toggle_igi` (rtw8822c.c:2257): the initial gain of both
//! paths (`REG_RXIGI`, 0x1D70: path A in bits 6:0, B in 14:8) stepped down two
//! and back, which makes the AGC take the value again after a retune.

use super::bits::{read32_mask, write32_mask};
use crate::regs::Mmio;

const REG_RXIGI: usize = 0x1D70;
const IGI_A: u32 = 0x7F;
const IGI_B: u32 = 0x7F00;

pub fn toggle_igi<M: Mmio>(mmio: &M) {
    let igi = read32_mask(mmio, REG_RXIGI, IGI_A);
    write32_mask(mmio, REG_RXIGI, IGI_A, igi.wrapping_sub(2));
    write32_mask(mmio, REG_RXIGI, IGI_B, igi.wrapping_sub(2));
    write32_mask(mmio, REG_RXIGI, IGI_A, igi);
    write32_mask(mmio, REG_RXIGI, IGI_B, igi);
}
