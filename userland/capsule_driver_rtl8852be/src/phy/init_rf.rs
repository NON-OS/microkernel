// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_phy_init_rf_reg` (Linux v6.12 rtw89 `phy.c`) for the 8852B, whose
//! radio tables use `rtw89_phy_config_rf_reg_v1`: every entry is written
//! with the full RF mask, and those at direct addresses (0x100 and up) are
//! also kept, as `(addr << 20) | data`, for the firmware, which gets them
//! per path in `rtw89_phy_config_rf_reg_fw`'s pages. Past three pages
//! rtw89 warns, keeps no more, and sends none; so does this.

use super::bb::Bb;
use super::rf::RFREG_MASK;
use super::table::{walk, TableFail};
use crate::regs::Mmio;
use alloc::vec::Vec;

pub const RADIO_A: &[u8] = include_bytes!("../../tables/radioa.bin");
pub const RADIO_B: &[u8] = include_bytes!("../../tables/radiob.bin");
/// `RTW89_H2C_RF_PAGE_NUM * RTW89_H2C_RF_PAGE_SIZE`.
pub const FW_WORDS_MAX: usize = 3 * 500;

/// What one path's table left for the firmware.
#[derive(Debug, PartialEq, Eq)]
pub enum ForFw {
    Words(Vec<u32>),
    /// More than `FW_WORDS_MAX` words: none are sent.
    TooMany,
}

/// Write `path`'s radio table; returns the words for the firmware.
pub fn init_rf(r: &impl Mmio, path: u8, rfe: u8, cv: u8) -> Result<ForFw, TableFail> {
    let bb = Bb(r);
    let table = if path == 0 { RADIO_A } else { RADIO_B };
    let (mut words, mut seen) = (Vec::new(), 0usize);
    walk(table, rfe, cv, |a, d| {
        bb.write_rf(path, a, RFREG_MASK, d);
        if a >= 0x100 {
            seen += 1;
            if words.len() < FW_WORDS_MAX {
                words.push((a << 20) | d);
            }
        }
    })?;
    Ok(if seen > FW_WORDS_MAX { ForFw::TooMany } else { ForFw::Words(words) })
}
