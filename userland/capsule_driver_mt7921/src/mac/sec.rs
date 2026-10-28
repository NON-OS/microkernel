/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the chip did to a protected frame, read as Linux v6.12
//! `mt7921_mac_fill_rx` reads RXD1: a cipher in `SEC_MODE` (bits 20:16)
//! with neither `CM` nor `CLM` (bits 23, 24) means decrypted, its CCMP
//! header and MIC stripped (`RX_FLAG_DECRYPTED | RX_FLAG_IV_STRIPPED |
//! RX_FLAG_MIC_STRIPPED`), its packet number in group 1's first six
//! octets, PN0 first, and its key index in `KEY_ID` (bits 22:21). No
//! cipher, or a mismatch, means the chip left the frame as it came, and
//! mac80211 opens it in software; so does the station. Only CCMP-128 is
//! ever installed here, so any other cipher is refused. A frame without
//! Protected is never one the chip decrypted: a BIP frame keeps its MMIE.

/// `MT_CIPHER_AES_CCMP` (`enum mt76_cipher_type`).
const AES_CCMP: u32 = 4;
const CM_CLM: u32 = 3 << 23;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sec {
    /// As it came over the air.
    Plain,
    /// Decrypted under the key at this index; group 1 has its number.
    Decrypted(u8),
    Refused,
}

pub fn judge(d1: u32, protected: bool) -> Sec {
    let mode = d1 >> 16 & 0x1f;
    if !protected || mode == 0 || d1 & CM_CLM != 0 {
        Sec::Plain
    } else if mode == AES_CCMP {
        Sec::Decrypted((d1 >> 21 & 3) as u8)
    } else {
        Sec::Refused
    }
}

/// The packet number group 1 opens with.
pub fn pn(g1: &[u8]) -> u64 {
    g1.iter().take(6).rev().fold(0, |n, b| n << 8 | u64::from(*b))
}

/// A decrypted `frame` put back as received for the station, as
/// `mt76_insert_ccmp_hdr` puts back a fragment's header: the MAC header,
/// the CCMP header at `pn` and `key_id`, the plaintext, then eight zero
/// octets where the MIC the chip checked was. Its length in `out`.
pub fn restore(frame: &[u8], hdr: usize, pn: u64, key_id: u8, out: &mut [u8]) -> Option<usize> {
    let n = frame.len().checked_add(16)?;
    let (head, body) = (frame.get(..hdr)?, &frame[hdr..]);
    let out = out.get_mut(..n)?;
    let p = pn.to_le_bytes();
    out[..hdr].copy_from_slice(head);
    let ccmp = [p[0], p[1], 0, 0x20 | key_id << 6, p[2], p[3], p[4], p[5]];
    out[hdr..hdr + 8].copy_from_slice(&ccmp);
    out[hdr + 8..n - 8].copy_from_slice(body);
    out[n - 8..].fill(0);
    Some(n)
}
