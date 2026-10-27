// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_set_channel_bb` (rtw8822c.c:2268) for a 2.4 GHz channel at
//! 20 MHz: CCK on, the RX AGC tables, the spur tracking by channel, the CCK
//! transmit filter (its own for channel 14), the DFIR, and the 20 MHz
//! bandwidth settings. Register facts from rtw8822c.h.

use super::super::bits::{clr32, set32, write32_mask};
use crate::regs::Mmio;

const MASKDWORD: u32 = 0xFFFF_FFFF;
const MASKHWORD: u32 = 0xFFFF_0000;
const MASKLWORD: u32 = 0x0000_FFFF;

/// The CCK transmit filter, TXF0..TXF7: (register, mask, ch 14, others).
const TXF: [(usize, u32, u32, u32); 8] = [
    (0x1A20, MASKHWORD, 0x3DA0, 0x5284),
    (0x1A24, MASKDWORD, 0x4962_C931, 0x3E18_FEC8),
    (0x1A28, MASKLWORD, 0x6AA3, 0x0A88),
    (0x1A98, MASKHWORD, 0xAA7B, 0xACC4),
    (0x1A9C, MASKLWORD, 0xF3D7, 0xC8B2),
    (0x1AA0, MASKDWORD, 0x0000_0000, 0x00FA_F0DE),
    (0x1AAC, MASKDWORD, 0xFF01_2455, 0x0012_2344),
    (0x1AB0, MASKDWORD, 0x0000_FFFF, 0x0FFF_FFFF),
];

/// The spur-tracking value for `channel` (REG_SCOTRK, bits 11:0).
pub fn scotrk(channel: u8) -> u32 {
    match channel {
        13 | 14 => 0x969,
        11 | 12 => 0x96A,
        _ => 0x9AA,
    }
}

pub fn set_2g_20<M: Mmio>(mmio: &M, channel: u8) {
    clr32(mmio, 0x1A14, (1 << 8) | (1 << 9)); // REG_BGCTRL: BITS_RX_IQ_WEIGHT
    set32(mmio, 0x1A9C, 1 << 20); // REG_TXF4
    clr32(mmio, 0x0454, 1 << 7); // REG_CCK_CHECK: BIT_CHECK_CCK_EN
    clr32(mmio, 0x1A80, 1 << 18); // REG_CCKTXONLY: BIT_BB_CCK_CHECK_EN
    write32_mask(mmio, 0x1C80, 0x3F00_0000, 0xF); // REG_CCAMSK
    rx_agc_20(mmio);
    write32_mask(mmio, 0x0C30, 0xFFF, scotrk(channel)); // REG_SCOTRK
    for (reg, mask, ch14, other) in TXF {
        write32_mask(mmio, reg, mask, if channel == 14 { ch14 } else { other });
    }
    write32_mask(mmio, 0x0808, 0x70, if channel == 13 { 0x3 } else { 0x1 }); // REG_TXDFIR0
    bw_20(mmio);
}

/// RX AGC for 20 MHz: CCK (bits 15:12) 5, OFDM (bits 8:4) 6, both paths.
fn rx_agc_20<M: Mmio>(mmio: &M) {
    write32_mask(mmio, 0x18AC, 0xF000, 0x5); // REG_RXAGCCTL0
    write32_mask(mmio, 0x41AC, 0xF000, 0x5); // REG_RXAGCCTL
    write32_mask(mmio, 0x18AC, 0x1F0, 0x6);
    write32_mask(mmio, 0x41AC, 0x1F0, 0x6);
}

/// RTW_CHANNEL_WIDTH_20.
fn bw_20<M: Mmio>(mmio: &M) {
    write32_mask(mmio, 0x0810, 0x3FF0, 0x19B); // REG_DFIRBW
    write32_mask(mmio, 0x09B0, 0xF, 0x0); // REG_TXBWCTL
    write32_mask(mmio, 0x09B0, 0xFFC0, 0x0);
    write32_mask(mmio, 0x09B4, 0x700, 0x7); // REG_TXCLK
    write32_mask(mmio, 0x09B4, 0x70_0000, 0x6);
    write32_mask(mmio, 0x1ABC, 1 << 30, 0x0); // REG_CCK_SOURCE: BIT_NBI_EN
    write32_mask(mmio, 0x088C, 0xF000, 0x1); // REG_SBD: BITS_SUBTUNE
    write32_mask(mmio, 0x0CBC, 1 << 21, 0x0); // REG_PT_CHSMO: BIT_PT_OPT
}
