/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! How each piece is announced to the chip before its bytes go: the
//! download mode word and the command, as Linux v6.12
//! `mt76_connac2_get_data_mode` (patch sections, the MT7921 family's
//! branch), `mt76_connac_mcu_gen_dl_mode` (RAM regions, not WA) and
//! `mt76_connac_mcu_init_download` choose them.

use crate::mcu::cmd::{Cmd, PATCH_START_REQ, TARGET_ADDRESS_LEN_REQ};

pub const DL_MODE_ENCRYPT: u32 = 1 << 0;
pub const DL_MODE_KEY_IDX: u32 = 0x3 << 1;
pub const DL_MODE_RESET_SEC_IV: u32 = 1 << 3;
pub const DL_CONFIG_ENCRY_MODE_SEL: u32 = 1 << 6;
pub const DL_MODE_NEED_RSP: u32 = 1 << 31;

/// A patch section's mode from its `sec_key_idx`: plain, AES with the key
/// index in the low byte, or scrambled. An unknown encryption is sent as
/// plain, as Linux does after logging it.
pub fn patch_mode(info: u32) -> u32 {
    let mut mode = DL_MODE_NEED_RSP;
    if info == u32::MAX {
        return mode; // PATCH_SEC_NOT_SUPPORT
    }
    match info >> 24 {
        0x01 => mode |= DL_MODE_ENCRYPT | ((info & 0xff) << 1 & DL_MODE_KEY_IDX),
        0x02 => mode |= DL_MODE_ENCRYPT | DL_CONFIG_ENCRY_MODE_SEL,
        _ => return mode,
    }
    mode | DL_MODE_RESET_SEC_IV
}

/// A RAM region's mode from its feature byte: `FW_FEATURE_SET_ENCRYPT`
/// (bit 0), `FW_FEATURE_SET_KEY_IDX` (bits 2:1), `FW_FEATURE_ENCRY_MODE`
/// (bit 4).
pub fn ram_mode(feature: u8) -> u32 {
    let f = u32::from(feature);
    let mut mode = DL_MODE_NEED_RSP | (f >> 1 & 0x3) << 1;
    if f & 1 != 0 {
        mode |= DL_MODE_ENCRYPT | DL_MODE_RESET_SEC_IV;
    }
    if f & (1 << 4) != 0 {
        mode |= DL_CONFIG_ENCRY_MODE_SEL;
    }
    mode
}

/// The announcement for bytes bound for `addr`: the patch area's start
/// (`MCU_PATCH_ADDRESS`, or the MT7921 family's 0x900000) takes
/// `PATCH_START_REQ`, any other `TARGET_ADDRESS_LEN_REQ`.
pub fn request_for(addr: u32) -> Cmd {
    if addr == 0x200000 || addr == 0x900000 {
        PATCH_START_REQ
    } else {
        TARGET_ADDRESS_LEN_REQ
    }
}
