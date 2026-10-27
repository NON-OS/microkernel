/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A command as the MCU reads it off ring 17: the hardware TXD, then the
//! connac2 command header, then the body. Linux v6.12
//! `mt76_connac2_mcu_fill_message` and `struct mt76_connac2_mcu_txd` /
//! `mt76_connac2_mcu_uni_txd` of `mt76_connac_mcu.h`; the TXD bits are
//! `mt76_connac2_mac.h`'s.

use super::cmd::{Cmd, Kind, EXT_CID};

/// The hardware TXD: eight words.
pub const TXD_LEN: usize = 32;
/// The plain, extended and offload header, after the TXD.
pub const HDR_LEN: usize = TXD_LEN + 32;
/// The unified header, after the TXD.
pub const UNI_HDR_LEN: usize = TXD_LEN + 16;

const PKT_TYPE_CMD: u32 = 2; // MT_TX_TYPE_CMD, in TXD0 bits 24:23
const Q_IDX_MCU_RX_Q0: u32 = 0x20; // MT_TX_MCU_PORT_RX_Q0, TXD0 bits 31:25
const LONG_FORMAT: u32 = 1 << 31;
const HDR_FORMAT_CMD: u32 = 1 << 16; // MT_HDR_FORMAT_CMD in TXD1 bits 17:16
const MCU_PKT_ID: u8 = 0xa0;
/// `MCU_PQ_ID(MT_TX_PORT_IDX_MCU, MT_TX_MCU_PORT_RX_Q0)`: `1 << 15` and
/// `0x20 << 10`, the same bit.
const PQ_ID: u16 = 0x8000;
const UNI_EXT_ACK: u8 = 0x07; // MCU_CMD_ACK | MCU_CMD_UNI | MCU_CMD_SET
const Q_QUERY: u8 = 0;
const Q_SET: u8 = 1;
const Q_NA: u8 = 3;

/// The header's length for `cmd`.
pub fn header_len(cmd: Cmd) -> usize {
    if cmd.kind == Kind::Uni {
        UNI_HDR_LEN
    } else {
        HDR_LEN
    }
}

/// `cmd` with sequence `seq` and `body` into `out`; the length, or `None`
/// when `out` is too short or the length does not fit sixteen bits.
pub fn frame(cmd: Cmd, seq: u8, body: &[u8], out: &mut [u8]) -> Option<usize> {
    let hl = header_len(cmd);
    let total = hl.checked_add(body.len()).filter(|&t| t <= u16::MAX as usize)?;
    let out = out.get_mut(..total)?;
    out[..hl].fill(0);
    out[hl..].copy_from_slice(body);
    let txd0 = total as u32 | PKT_TYPE_CMD << 23 | Q_IDX_MCU_RX_Q0 << 25;
    out[0..4].copy_from_slice(&txd0.to_le_bytes());
    out[4..8].copy_from_slice(&(LONG_FORMAT | HDR_FORMAT_CMD).to_le_bytes());
    let len = ((total - TXD_LEN) as u16).to_le_bytes();
    out[32..34].copy_from_slice(&len);
    if cmd.kind == Kind::Uni {
        out[34..36].copy_from_slice(&u16::from(cmd.id).to_le_bytes());
        (out[37], out[39], out[43]) = (MCU_PKT_ID, seq, UNI_EXT_ACK);
        return Some(total);
    }
    out[34..36].copy_from_slice(&PQ_ID.to_le_bytes());
    let (cid, ext) = if cmd.kind == Kind::Ext { (EXT_CID, cmd.id) } else { (cmd.id, 0) };
    let set_query = match (cmd.kind, cmd.query) {
        (Kind::Ext | Kind::Ce, true) => Q_QUERY,
        (Kind::Ext | Kind::Ce, false) => Q_SET,
        _ => Q_NA,
    };
    (out[36], out[37], out[38], out[39]) = (cid, MCU_PKT_ID, set_query, seq);
    (out[41], out[43]) = (ext, u8::from(ext != 0));
    Some(total)
}
