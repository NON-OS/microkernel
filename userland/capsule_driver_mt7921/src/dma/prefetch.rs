/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The rings' prefetch windows: Linux v6.12 `mt792x_dma_prefetch`, the
//! branch every chip but the MT7925 takes, as `(register, PREFETCH(base,
//! depth))`: receive rings 0, 2, 3, 4, 5, then transmit 0 to 6, 16, 17.

use crate::hw::addr::{RX_RING_EXT_CTRL, TX_RING_EXT_CTRL};

pub const PREFETCH: [(u32, u32); 14] = [
    (RX_RING_EXT_CTRL, 0x4),
    (RX_RING_EXT_CTRL + 8, 0x40 << 16 | 0x4),
    (RX_RING_EXT_CTRL + 12, 0x80 << 16 | 0x4),
    (RX_RING_EXT_CTRL + 16, 0xc0 << 16 | 0x4),
    (RX_RING_EXT_CTRL + 20, 0x100 << 16 | 0x4),
    (TX_RING_EXT_CTRL, 0x140 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 4, 0x180 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 8, 0x1c0 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 12, 0x200 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 16, 0x240 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 20, 0x280 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 24, 0x2c0 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 64, 0x340 << 16 | 0x4),
    (TX_RING_EXT_CTRL + 68, 0x380 << 16 | 0x4),
];
