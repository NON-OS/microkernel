/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Which host rings the driver runs, where their registers are, and how big
//! each is. The ring numbers are Linux v6.12 `mt7921_dma_init`'s: transmit
//! ring 0 for frames, 16 for the firmware download, 17 for commands to the
//! WM firmware; receive ring 0 for events before the firmware runs, ring 4
//! (`MT_WFDMA0(0x540)`) for events after, ring 2 (`MT_RX_DATA_RING_BASE`)
//! for frames. The sizes are this driver's, far below Linux's: it polls one
//! request at a time, and each ring fits one broker grant of 64 pages.

use crate::hw::addr::{RX_RING_BASE, TX_RING_BASE};

/// A ring: its first register, its slots, and the bytes of each slot.
#[derive(Clone, Copy)]
pub struct Shape {
    pub reg: u32,
    pub n: u16,
    pub stride: usize,
}

const fn tx(ring: u32, n: u16, stride: usize) -> Shape {
    Shape { reg: TX_RING_BASE + ring * 16, n, stride }
}

const fn rx(ring: u32, n: u16, stride: usize) -> Shape {
    Shape { reg: RX_RING_BASE + ring * 16, n, stride }
}

/// `MT_RX_BUF_SIZE`: every receive buffer.
pub const RX_BUF: usize = 2048;
/// A frame's TXD (32 bytes) and TXP (24 bytes), one data-ring slot each.
pub const TXWI_SLOT: usize = 64;
/// The largest chunk of a firmware download (`max_len` of
/// `mt76_connac2_load_patch` and `mt76_connac_mcu_send_ram_firmware`).
pub const FW_CHUNK: usize = 4096;
/// The largest command: its header and the biggest body the driver builds
/// (a transmit power message, 44 + 8 x 162 bytes).
pub const CMD_MAX: usize = 2048;
/// Frame buffers, each held until the chip reports the frame sent.
pub const TOKENS: usize = 64;
pub const TOKEN_BUF: usize = 2048;

pub const TX_DATA: Shape = tx(0, 64, TXWI_SLOT);
pub const TX_FWDL: Shape = tx(16, 16, FW_CHUNK);
pub const TX_WM: Shape = tx(17, 16, CMD_MAX);
pub const RX_MCU: Shape = rx(0, 8, RX_BUF);
pub const RX_DATA: Shape = rx(2, 64, RX_BUF);
pub const RX_WA: Shape = rx(4, 32, RX_BUF);

/// Every ring's descriptors share one page, each array at its own offset.
pub const ALL: [Shape; 6] = [TX_DATA, TX_FWDL, TX_WM, RX_MCU, RX_DATA, RX_WA];
pub const DESC_PAGE: usize = 4096;

/// Where shape `i` of `ALL` keeps its descriptors in the shared page.
pub fn desc_offset(i: usize) -> usize {
    ALL[..i].iter().map(|s| (s.n as usize * 16).next_multiple_of(256)).sum()
}

// The descriptor arrays fit their page.
const _: () = assert!(64 * 16 + 256 + 256 + 256 + 64 * 16 + 32 * 16 <= DESC_PAGE);
