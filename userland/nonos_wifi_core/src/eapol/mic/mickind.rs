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

use super::*;

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
