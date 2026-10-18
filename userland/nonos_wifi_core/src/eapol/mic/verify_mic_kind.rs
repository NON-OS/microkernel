// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Whether the MIC carried in `frame` matches the one computed under `kck`
//! with the given algorithm, bounded to the announced length as
//! `verify_mic`.

use super::*;

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
