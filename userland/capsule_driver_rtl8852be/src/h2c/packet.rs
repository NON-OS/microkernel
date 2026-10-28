// NONOS Operating System (AGPL-3.0-or-later)

//! A command packet for the firmware queue: the WD body rtw89 puts before
//! it (`rtw89_core_fill_txdesc` with `rtw89_core_tx_update_h2c_info`:
//! channel 12, no WD page, the packet's size), and the 8-byte H2C header
//! (`rtw89_h2c_pkt_set_hdr` in Linux v6.12 rtw89 `fw.c`).

use crate::regs::field;

pub const WD_BODY: usize = 24;
pub const H2C_HDR: usize = 8;
const BODY0_CHANNEL_DMA: u32 = 0xf_0000;
const BODY0_FW_DL: u32 = 0x10_0000;
const BODY2_TXPKT_SIZE: u32 = 0x3fff;
const BODY3_SW_SEQ: u32 = 0xfff;
/// `RTW89_DMA_H2C`.
pub const CH_H2C: u32 = 12;

/// The WD body for a firmware-queue packet of `size` bytes. rtw89 fills the
/// software sequence from where an 802.11 header's sequence control would
/// sit in the packet (bytes 22 and 23); the firmware queue carries no
/// 802.11 header, and it is reproduced as rtw89 writes it.
pub fn fwcmd_wd(pkt: &[u8], fw_dl: bool) -> [u8; WD_BODY] {
    let seq = match pkt.get(22..24) {
        Some(s) => (u16::from_le_bytes([s[0], s[1]]) & 0xfff0) >> 4,
        None => 0,
    };
    let mut wd = [0u8; WD_BODY];
    let d0 = field(BODY0_CHANNEL_DMA, CH_H2C) | if fw_dl { BODY0_FW_DL } else { 0 };
    wd[..4].copy_from_slice(&d0.to_le_bytes());
    wd[8..12].copy_from_slice(&field(BODY2_TXPKT_SIZE, pkt.len() as u32).to_le_bytes());
    wd[12..16].copy_from_slice(&field(BODY3_SW_SEQ, seq as u32).to_le_bytes());
    wd
}

/// Where a command goes: category, class, function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd {
    pub cat: u8,
    pub class: u8,
    pub func: u8,
}

/// The H2C header for `len` bytes of command: type H2C, the sequence, and
/// the receive and done acknowledgements asked for.
pub fn h2c_hdr(cmd: Cmd, seq: u8, len: usize, rack: bool, dack: bool) -> [u8; H2C_HDR] {
    let h0 = field(0x3, cmd.cat as u32)
        | field(0xfc, cmd.class as u32)
        | field(0xff00, cmd.func as u32)
        | field(0xff00_0000, seq as u32);
    let h1 = field(0x3fff, (len + H2C_HDR) as u32)
        | if rack { 1 << 14 } else { 0 }
        | if dack { 1 << 15 } else { 0 };
    let mut h = [0u8; H2C_HDR];
    h[..4].copy_from_slice(&h0.to_le_bytes());
    h[4..].copy_from_slice(&h1.to_le_bytes());
    h
}
