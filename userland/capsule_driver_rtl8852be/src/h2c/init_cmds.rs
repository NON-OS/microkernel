// NONOS Operating System (AGPL-3.0-or-later)

//! The commands the MAC and PHY init send (Linux v6.12 rtw89 `fw.c`):
//! `rtw89_fw_h2c_set_ofld_cfg`, `rtw89_fw_h2c_fw_log` with the log left
//! off (rtw89's default), and `rtw89_fw_h2c_rf_reg`, the RF table's
//! registers handed to the firmware in pages of 500 words per path.

use super::packet::Cmd;
use super::send::{h2c, SendFail};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::TxRing;

pub const OFLD_CFG: Cmd = Cmd { cat: 1, class: 9, func: 0x14 };
pub const OFLD_CFG_BODY: [u8; 8] = [0x09, 0, 0, 0, 0x5e, 0, 0, 0];
pub const LOG_CFG: Cmd = Cmd { cat: 1, class: 0, func: 0 };
/// Level LOUD (4), path C2H (bit 1), no components.
pub const LOG_CFG_OFF: [u8; 12] = [4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/// `RTW89_H2C_RF_PAGE_SIZE` words a page (`init_rf::FW_WORDS_MAX` caps
/// the pages at `RTW89_H2C_RF_PAGE_NUM`, 3).
pub const RF_PAGE_WORDS: usize = 500;

pub fn ofld_cfg<M: DmaMem>(r: &impl Mmio, q: &mut TxRing<M>, fw: &mut Fw) -> Result<(), SendFail> {
    h2c(r, q, fw, OFLD_CFG, &OFLD_CFG_BODY, true)
}

pub fn fw_log_off<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
) -> Result<(), SendFail> {
    h2c(r, q, fw, LOG_CFG, &LOG_CFG_OFF, false)
}

/// One page of RF register words (`(addr << 20) | data`) for `path`.
pub fn rf_reg_page<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    path: u8,
    page: u8,
    words: &[u32],
) -> Result<(), SendFail> {
    let cmd = Cmd { cat: 2, class: if path == 0 { 8 } else { 9 }, func: page };
    let mut body = [0u8; RF_PAGE_WORDS * 4];
    let n = words.len().min(RF_PAGE_WORDS);
    for (i, w) in words[..n].iter().enumerate() {
        body[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    h2c(r, q, fw, cmd, &body[..n * 4], false)
}
