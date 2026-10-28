// NONOS Operating System (AGPL-3.0-or-later)

//! The indexes written as the 8822C takes them (rtw88
//! `rtw8822c_set_write_tx_power_ref` and `rtw8822c_set_tx_power_diff`):
//! each path's CCK reference (its 11M index) and OFDM reference (its MCS7
//! index), then per group of four rates the smallest diff from the reference
//! over the paths, 7 bits a rate, at 0x3A00 plus the group's first rate.

use super::super::bits::write32_mask;
use super::index::{cap, indexes, Indexes};
use crate::efuse::facts::PathPower;
use crate::regs::Mmio;

const REG_TXREF_CCK: [usize; 2] = [0x18A0, 0x41A0];
const REG_TXREF_OFDM: [usize; 2] = [0x18E8, 0x41E8];
/// 0x1C90 bit 15, cleared before every TX AGC write.
const REG_TXAGC_CTRL: usize = 0x1C90;
const REG_TXAGC_DIFF: usize = 0x3A00;

/// The rate groups written: CCK 1-11M, OFDM 6-24M and 36-54M, MCS0-3 and
/// MCS4-7, each with its diff from its family's reference.
const GROUPS: [(u8, fn(&Indexes) -> i16); 5] = [
    (0x00, |_| 0),
    (0x04, |i| i16::from(i.ofdm) - i16::from(i.ht)),
    (0x08, |i| i16::from(i.ofdm) - i16::from(i.ht)),
    (0x0C, |_| 0),
    (0x10, |_| 0),
];

/// The diff word for the group: the smallest diff over `idx`, four times.
pub fn diff_word(idx: &[Indexes], diff: fn(&Indexes) -> i16) -> u32 {
    let d = idx.iter().map(diff).min().unwrap_or(0);
    u32::from_le_bytes([(d as u8) & 0x7F; 4])
}

/// Write both paths' references and the rate diffs for `power`.
pub fn set_tx_power<M: Mmio>(mmio: &M, power: &[PathPower; 2], paths: usize) {
    let cap = cap();
    let idx = [indexes(&power[0], cap), indexes(&power[1], cap)];
    let idx = &idx[..paths.clamp(1, 2)];
    for (p, i) in idx.iter().enumerate() {
        write32_mask(mmio, REG_TXAGC_CTRL, 1 << 15, 0);
        write32_mask(mmio, REG_TXREF_CCK[p], 0x7F_0000, u32::from(i.cck));
    }
    for (p, i) in idx.iter().enumerate() {
        write32_mask(mmio, REG_TXAGC_CTRL, 1 << 15, 0);
        write32_mask(mmio, REG_TXREF_OFDM[p], 0x1_FC00, u32::from(i.ht));
    }
    for (rate, diff) in GROUPS {
        write32_mask(mmio, REG_TXAGC_CTRL, 1 << 15, 0);
        write32_mask(mmio, REG_TXAGC_DIFF + usize::from(rate), 0xFFFF_FFFF, diff_word(idx, diff));
    }
}
