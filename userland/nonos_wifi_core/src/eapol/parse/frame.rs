// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Parse an EAPOL-Key frame.

use super::*;

/// Parse an EAPOL-Key frame. Returns `None` if it is too short, is not a Key
/// frame, or the key-data length runs past the buffer.
pub fn parse(frame: &[u8]) -> Option<EapolKey<'_>> {
    if frame.len() < HEADER_LEN || frame[1] != EAPOL_TYPE_KEY {
        return None;
    }
    let key_data_len = u16::from_be_bytes([frame[97], frame[98]]) as usize;
    let end = HEADER_LEN.checked_add(key_data_len)?;
    if end > frame.len() {
        return None;
    }
    let mut replay_counter = [0u8; 8];
    let mut nonce = [0u8; 32];
    let mut key_rsc = [0u8; 8];
    let mut mic = [0u8; 16];
    replay_counter.copy_from_slice(&frame[9..17]);
    nonce.copy_from_slice(&frame[17..49]);
    key_rsc.copy_from_slice(&frame[65..73]);
    mic.copy_from_slice(&frame[MIC_OFFSET..MIC_OFFSET + MIC_LEN]);
    Some(EapolKey {
        descriptor_type: frame[4],
        key_info: u16::from_be_bytes([frame[5], frame[6]]),
        key_length: u16::from_be_bytes([frame[7], frame[8]]),
        replay_counter,
        nonce,
        key_rsc,
        mic,
        key_data: &frame[HEADER_LEN..end],
    })
}
