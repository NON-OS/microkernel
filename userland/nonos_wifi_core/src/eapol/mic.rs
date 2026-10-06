// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Verify and compute the MIC of an EAPOL-Key frame. The MIC covers the whole
//! frame with the 16-byte MIC field zeroed, keyed by the KCK (the first 16
//! bytes of the PTK). Key descriptor version 2 (WPA2-PSK) takes the first 16
//! bytes of HMAC-SHA1; version 3 and the SAE AKM take AES-128-CMAC (IEEE Std
//! 802.11-2020, 12.7.3). The HMAC and CMAC underneath are checked against RFC
//! vectors, so a round-trip and a tamper test pin the verify logic exactly.

use super::parse::{HEADER_LEN, MIC_LEN, MIC_OFFSET};
use crate::ccmp::cmac::aes_cmac_parts;
use crate::wpa::hmac::hmac_sha1_parts;

/// Which algorithm computes the EAPOL-Key MIC.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MicKind {
    /// HMAC-SHA1 truncated to 128 bits (key descriptor version 2).
    HmacSha1,
    /// AES-128-CMAC (key descriptor version 3, and the SAE AKM).
    AesCmac,
}

/// Compute the version-2 MIC of `frame` under `kck`. A frame too short to hold
/// the MIC field yields an all-zero MIC, which no verify accepts.
pub fn compute_mic(kck: &[u8], frame: &[u8]) -> [u8; MIC_LEN] {
    compute_mic_kind(MicKind::HmacSha1, kck, frame)
}

/// Compute the MIC of `frame` under `kck` with the given algorithm.
pub fn compute_mic_kind(kind: MicKind, kck: &[u8], frame: &[u8]) -> [u8; MIC_LEN] {
    let mut mic = [0u8; MIC_LEN];
    let Some(tail) = frame.get(MIC_OFFSET + MIC_LEN..) else {
        return mic;
    };
    let zeros = [0u8; MIC_LEN];
    let parts: [&[u8]; 3] = [&frame[..MIC_OFFSET], &zeros, tail];
    match kind {
        MicKind::HmacSha1 => {
            let h = hmac_sha1_parts(kck, &parts);
            mic.copy_from_slice(&h[..MIC_LEN]);
        }
        MicKind::AesCmac => {
            let Ok(key) = <[u8; 16]>::try_from(kck) else {
                return mic;
            };
            mic = aes_cmac_parts(&key, &parts);
        }
    }
    mic
}

/// Whether the version-2 MIC carried in `frame` matches the one computed
/// under `kck`. The comparison is constant time.
///
/// The MIC covers exactly the EAPOL-Key frame: the fixed header plus the key
/// data its length field announces. A received MPDU can carry a trailing FCS or
/// padding past that, which is not part of the MIC input, so the frame is bounded
/// to the announced length before hashing rather than trusting `frame.len()`.
pub fn verify_mic(kck: &[u8], frame: &[u8]) -> bool {
    verify_mic_kind(MicKind::HmacSha1, kck, frame)
}

/// Whether the MIC carried in `frame` matches the one computed under `kck`
/// with the given algorithm, bounded to the announced length as `verify_mic`.
pub fn verify_mic_kind(kind: MicKind, kck: &[u8], frame: &[u8]) -> bool {
    // A CMAC key that is not 16 bytes computes no MIC at all; refuse it here
    // rather than compare against a MIC of zeros that was never computed.
    if frame.len() < HEADER_LEN || (kind == MicKind::AesCmac && kck.len() != 16) {
        return false;
    }
    let key_data_len = u16::from_be_bytes([frame[HEADER_LEN - 2], frame[HEADER_LEN - 1]]) as usize;
    let end = match HEADER_LEN.checked_add(key_data_len) {
        Some(e) if e <= frame.len() => e,
        _ => return false,
    };
    let computed = compute_mic_kind(kind, kck, &frame[..end]);
    let mut diff = 0u8;
    for (a, b) in frame[MIC_OFFSET..MIC_OFFSET + MIC_LEN].iter().zip(computed.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}
