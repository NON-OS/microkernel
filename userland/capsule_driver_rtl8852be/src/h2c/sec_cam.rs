/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A security CAM entry, as Linux v6.12 rtw89 writes one for a key
//! (`rtw89_cam_get_sec_key_cmd` in `cam.c`, the `RTW89_SET_FWCMD_SEC_*`
//! fields of `fw.h`): `H2C_CL_MAC_SEC_CAM`, `H2C_FUNC_MAC_SEC_UPD`, 24
//! octets (`H2C_SEC_CAM_LEN`): the entry's index, offset 0 and length
//! `RTW89_SEC_CAM_LEN`; its type (`RTW89_SEC_KEY_TYPE_*`), no extended key,
//! no SPP; then the key's sixteen octets as four little-endian words.

use super::packet::Cmd;

pub const SEC_UPD: Cmd = Cmd { cat: 1, class: 0xa, func: 1 };
pub const SEC_CAM_LEN: usize = 24;
/// `RTW89_SEC_CAM_LEN`.
const ENTRY_LEN: u32 = 20;
/// `RTW89_SEC_KEY_TYPE_CCMP128`, `RTW89_SEC_KEY_TYPE_BIP_CCMP128`.
pub const TYPE_CCMP128: u8 = 6;
pub const TYPE_BIP_CCMP128: u8 = 10;

/// Entry `idx` holding `key` of `kind`.
pub fn sec_body(idx: u8, kind: u8, key: &[u8; 16]) -> [u8; SEC_CAM_LEN] {
    let mut b = [0u8; SEC_CAM_LEN];
    b[..4].copy_from_slice(&(u32::from(idx) | ENTRY_LEN << 16).to_le_bytes());
    b[4] = kind & 0xf;
    b[8..].copy_from_slice(key);
    b
}
