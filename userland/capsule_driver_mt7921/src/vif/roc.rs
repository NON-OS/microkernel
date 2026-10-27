/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Remain on channel, `MCU_UNI_CMD_ROC`: how a channel is asked of the
//! firmware's channel manager (Linux v6.12 `mt7921_mcu_set_roc` and
//! `mt7921_mcu_abort_roc`). A 4-byte header, then `UNI_ROC_ACQUIRE` (tag
//! 0, 24 bytes): the BSS, a token the grant will echo, the channel at 20
//! MHz, the band (1 for 2.4 GHz, 2 for 5), the request's type (`JOIN` 0
//! before an association, `ROC` 1 to listen), how long in milliseconds,
//! and any DBDC band (0xff); or `UNI_ROC_ABORT` (tag 1, 12 bytes).

use super::BSS_IDX;

/// `MT7921_ROC_REQ_JOIN` and `MT7921_ROC_REQ_ROC`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Req {
    Join = 0,
    Listen = 1,
}

pub fn acquire(token: u8, channel: u8, ghz5: bool, req: Req, ms: u32) -> [u8; 28] {
    let mut b = [0u8; 28];
    b[6..8].copy_from_slice(&24u16.to_le_bytes());
    (b[8], b[9], b[10], b[11]) = (BSS_IDX, token, channel, 0);
    // band, CMD_CBW_20MHZ, centre, centre2, the AP's width and centres.
    (b[12], b[13], b[14], b[16], b[17]) = (if ghz5 { 2 } else { 1 }, 0, channel, 0, channel);
    b[19] = req as u8;
    b[20..24].copy_from_slice(&ms.to_le_bytes());
    b[24] = 0xff;
    b
}

pub fn abort(token: u8) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[4..6].copy_from_slice(&1u16.to_le_bytes());
    b[6..8].copy_from_slice(&12u16.to_le_bytes());
    (b[8], b[9], b[10]) = (BSS_IDX, token, 0xff);
    b
}
