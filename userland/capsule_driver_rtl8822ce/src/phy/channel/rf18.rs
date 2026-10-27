// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_set_channel_rf` (rtw8822c.c:2191): RF register 0x18 (band,
//! channel, synthesizer index, bandwidth) rewritten on both paths, with the
//! RX baseband filter's lookup entry for the bandwidth, all inside the
//! three-wire reset. The 8822C's bandwidth field is bits 13:12, where the
//! 8821C's is 11:10.

use super::super::bits::write32_mask;
use super::super::rf::{self, PATH_A, PATH_B, RFREG_MASK};
use crate::regs::Mmio;

const RF18_BAND_MASK: u32 = (1 << 16) | (1 << 9) | (1 << 8);
const RF18_BAND_5G: u32 = (1 << 16) | (1 << 8);
const RF18_CHANNEL_MASK: u32 = 0xFF;
const RF18_RFSI_MASK: u32 = (1 << 18) | (1 << 17);
const RF18_RFSI_GE_CH80: u32 = 1 << 17;
const RF18_RFSI_GT_CH140: u32 = 1 << 18;
const RF18_BW_MASK: u32 = (1 << 13) | (1 << 12);
const RF18_BW_20M: u32 = (1 << 13) | (1 << 12);
/// The RX baseband filter entry for 20 MHz.
const RXBB_20M: u32 = 0x18;

/// RF 0x18 for `channel` at 20 MHz, the rest of `old` kept.
pub fn rf18_value(channel: u8, old: u32) -> u32 {
    let mut v = old & !(RF18_BAND_MASK | RF18_CHANNEL_MASK | RF18_RFSI_MASK | RF18_BW_MASK);
    if channel > 14 {
        v |= RF18_BAND_5G;
    }
    v |= u32::from(channel) & RF18_CHANNEL_MASK;
    if (149..=177).contains(&channel) {
        v |= RF18_RFSI_GT_CH140;
    } else if (100..=144).contains(&channel) {
        v |= RF18_RFSI_GE_CH80;
    }
    v | RF18_BW_20M
}

pub fn set<M: Mmio>(mmio: &M, channel: u8) {
    let value = rf18_value(channel, rf::read(mmio, PATH_A, 0x18, RFREG_MASK));
    write32_mask(mmio, 0x1C90, 1 << 8, 0); // REG_RSTB: BIT_RSTB_3WIRE off
    for path in [PATH_A, PATH_B] {
        rf::write(mmio, path, 0xEE, 0x04, 1); // RF_LUTWE2
        rf::write(mmio, path, 0x33, 0x1F, 0x12); // RF_LUTWA
        rf::write(mmio, path, 0x3F, 0xF_FFFF, RXBB_20M); // RF_LUTWD0
        rf::write(mmio, path, 0xEE, 0x04, 0);
    }
    rf::write(mmio, PATH_A, 0x18, RFREG_MASK, value); // RF_CFGCH
    rf::write(mmio, PATH_B, 0x18, RFREG_MASK, value);
    write32_mask(mmio, 0x1C90, 1 << 8, 1);
    write32_mask(mmio, 0x1830, 1 << 29, 1); // REG_ANAPAR_A: BIT_ANAPAR_UPDATE
    write32_mask(mmio, 0x4130, 1 << 29, 1); // REG_ANAPAR_B
}
