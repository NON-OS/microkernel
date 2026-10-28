/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station's own MAC entry, `MCU_UNI_CMD_DEV_INFO_UPDATE` with its
//! `DEV_INFO_ACTIVE` TLV (Linux v6.12 `mt76_connac_mcu_uni_add_dev`):
//! `omac_idx`, `band_idx`, then tag 0, length 12, whether it is active,
//! the link index, and the address the chip answers and sends from.

use super::OMAC_IDX;

pub const LEN: usize = 16;

pub fn body(active: bool, addr: [u8; 6]) -> [u8; LEN] {
    let mut b = [0u8; LEN];
    b[0] = OMAC_IDX;
    b[6..8].copy_from_slice(&12u16.to_le_bytes());
    b[8] = u8::from(active);
    b[10..16].copy_from_slice(&addr);
    b
}
