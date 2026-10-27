/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Keys into the chip's WTBL, as Linux v6.12 `mt7921_set_key` sends them
//! through `mt76_connac_mcu_add_key`: `MCU_UNI_CMD_STA_REC_UPDATE` holding
//! the request header and one `STA_REC_KEY_V2` TLV, a `sta_rec_sec` of 80
//! octets put whole (`mt76_connac_mcu_sta_key_tlv`), its length cut to the
//! keys it carries. The pairwise key goes under the access point's entry;
//! the group key under the interface's own (`set_key` with no station takes
//! `mvif->sta`, MUAR 0xe), with the IGTK beside it under protected
//! management frames: the CCMP group key first, then `BIP_CMAC_128`, whose
//! index the TLV leaves zero. Linux sends no key removal on the MT7921
//! (`DISABLE_KEY` returns before the MCU): a key goes with its entry's
//! reset (`own::depart`).

use alloc::vec::Vec;

use super::{AP_WCID, BSS_IDX, OMAC_IDX, OWN_WCID};

/// `STA_REC_KEY_V2`, the eighteenth `STA_REC_*`.
const KEY_V2: u16 = 0x11;
/// `sizeof(struct sta_rec_sec)`: tag, length, add, count, two `sec_key`s.
const SEC_LEN: usize = 80;
/// `sizeof(struct sec_key)`: cipher, its length, index, key length, 32 octets.
const KEY_LEN: usize = 36;
/// `MCU_CIPHER_AES_CCMP`, `MCU_CIPHER_BIP_CMAC_128`.
pub const CIPHER_CCMP: u8 = 5;
pub const CIPHER_BIP_CMAC: u8 = 10;

/// The record's body for `wcid` (MUAR `muar`) setting `keys`: each its
/// cipher, index and key.
fn body(wcid: u16, muar: u8, keys: &[(u8, u8, &[u8; 16])]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + SEC_LEN);
    let w = wcid.to_le_bytes();
    out.extend_from_slice(&[BSS_IDX, w[0], 1, 0, 1, muar, w[1], 0]);
    let t = out.len();
    out.resize(t + SEC_LEN, 0);
    let len = (8 + KEY_LEN * keys.len()) as u16;
    out[t..t + 2].copy_from_slice(&KEY_V2.to_le_bytes());
    out[t + 2..t + 4].copy_from_slice(&len.to_le_bytes());
    // `add` is mac80211's `SET_KEY`, zero; then the count.
    out[t + 5] = keys.len() as u8;
    for (i, (cipher, id, key)) in keys.iter().enumerate() {
        let k = t + 8 + KEY_LEN * i;
        (out[k], out[k + 1], out[k + 2], out[k + 3]) = (*cipher, KEY_LEN as u8, *id, 16);
        out[k + 4..k + 20].copy_from_slice(&key[..]);
    }
    out
}

/// The pairwise key, index 0, under the access point's entry.
pub fn pairwise(tk: &[u8; 16]) -> Vec<u8> {
    body(AP_WCID, OMAC_IDX, &[(CIPHER_CCMP, 0, tk)])
}

/// The group key at `id`, and the IGTK with it, under the own entry.
pub fn group(id: u8, gtk: &[u8; 16], igtk: Option<&[u8; 16]>) -> Vec<u8> {
    match igtk {
        Some(i) => body(OWN_WCID, 0xe, &[(CIPHER_CCMP, id, gtk), (CIPHER_BIP_CMAC, 0, i)]),
        None => body(OWN_WCID, 0xe, &[(CIPHER_CCMP, id, gtk)]),
    }
}
