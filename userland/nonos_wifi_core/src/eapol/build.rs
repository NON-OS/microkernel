// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Build an EAPOL-Key frame, the send side of the four-way handshake. The
//! supplicant sends message two (its nonce) and message four (confirmation),
//! each carrying a MIC over the whole frame under the KCK. This lays out the
//! fixed header, appends any key data, and fills the MIC last.

use super::mic::{compute_mic_kind, MicKind};
use super::parse::{EAPOL_TYPE_KEY, HEADER_LEN, MIC_LEN, MIC_OFFSET};

/// The 802.1X protocol version the supplicant writes when it has no frame from
/// the authenticator to mirror (802.1X-2004).
pub const EAPOL_VERSION_DEFAULT: u8 = 2;

/// Build an EAPOL-Key frame into `out`, computing and inserting the version-2
/// (HMAC-SHA1) MIC under `kck`. Returns the frame length, or `None` if the
/// buffer is too small. The key-length field is left zero, as it is in the
/// supplicant's messages.
pub fn build_key_frame(
    out: &mut [u8],
    key_info: u16,
    replay_counter: &[u8; 8],
    nonce: &[u8; 32],
    key_data: &[u8],
    kck: &[u8],
) -> Option<usize> {
    let fields = KeyFrame { key_info, replay_counter, nonce, key_data };
    build_key_frame_kind(out, EAPOL_VERSION_DEFAULT, MicKind::HmacSha1, &fields, kck)
}

/// The fields of one EAPOL-Key frame the supplicant sends.
pub struct KeyFrame<'a> {
    pub key_info: u16,
    pub replay_counter: &'a [u8; 8],
    pub nonce: &'a [u8; 32],
    pub key_data: &'a [u8],
}

/// Build an EAPOL-Key frame with 802.1X protocol `version`, its MIC computed
/// with `kind` under `kck`. Returns the frame length, or `None` if the buffer
/// is too small or the key data cannot be described by the length fields.
pub fn build_key_frame_kind(
    out: &mut [u8],
    version: u8,
    kind: MicKind,
    f: &KeyFrame,
    kck: &[u8],
) -> Option<usize> {
    let total = HEADER_LEN.checked_add(f.key_data.len())?;
    let body_len = u16::try_from(total - 4).ok()?;
    let key_data_len = u16::try_from(f.key_data.len()).ok()?;
    if out.len() < total {
        return None;
    }
    out[..total].fill(0);
    out[0] = version;
    out[1] = EAPOL_TYPE_KEY;
    out[2..4].copy_from_slice(&body_len.to_be_bytes());
    out[4] = 2;
    out[5..7].copy_from_slice(&f.key_info.to_be_bytes());
    out[9..17].copy_from_slice(f.replay_counter);
    out[17..49].copy_from_slice(f.nonce);
    out[97..99].copy_from_slice(&key_data_len.to_be_bytes());
    out[HEADER_LEN..total].copy_from_slice(f.key_data);
    let mic = compute_mic_kind(kind, kck, &out[..total]);
    out[MIC_OFFSET..MIC_OFFSET + MIC_LEN].copy_from_slice(&mic);
    Some(total)
}
