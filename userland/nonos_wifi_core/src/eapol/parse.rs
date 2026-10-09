// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Parse an EAPOL-Key frame, the message of the WPA2 four-way handshake. The
//! layout is IEEE 802.1X / 802.11i: an EAPOL header, then the key descriptor
//! with the key-information flags, replay counter, nonce, MIC and key data.
//! Pure parsing over an attacker-controlled frame, checked by `iwlwifi_proofs`.

/// EAPOL packet type for a Key frame.
pub const EAPOL_TYPE_KEY: u8 = 0x03;
/// Offset of the 16-byte key MIC within the frame.
pub const MIC_OFFSET: usize = 81;
/// Length of the key MIC.
pub const MIC_LEN: usize = 16;
/// Length of the fixed EAPOL-Key header up to the key data.
pub const HEADER_LEN: usize = 99;

// Key-information flags (big-endian field at offset 5). The low three bits are
// the key-descriptor version; version 2 selects HMAC-SHA1 MIC + AES key wrap,
// which is what WPA2/CCMP uses.
pub const KEY_INFO_VERSION2: u16 = 2;
pub const KEY_INFO_PAIRWISE: u16 = 1 << 3;
pub const KEY_INFO_MIC: u16 = 1 << 8;
pub const KEY_INFO_SECURE: u16 = 1 << 9;
// The rest of the Key Information field (IEEE Std 802.11-2020, Figure 12-33):
// the version mask, the key index of the pre-RSN group exchange, Install, Key
// Ack, Error, Request and Encrypted Key Data. The supplicant checks each against
// what the message it is handling must carry.
pub const KEY_INFO_VERSION_MASK: u16 = 0x0007;
pub const KEY_INFO_INDEX_MASK: u16 = 0x0030;
pub const KEY_INFO_INSTALL: u16 = 1 << 6;
pub const KEY_INFO_ACK: u16 = 1 << 7;
pub const KEY_INFO_ERROR: u16 = 1 << 10;
pub const KEY_INFO_REQUEST: u16 = 1 << 11;
pub const KEY_INFO_ENCRYPTED: u16 = 1 << 12;
/// The descriptor type of an RSN (IEEE 802.11) EAPOL-Key frame.
pub const DESCRIPTOR_RSN: u8 = 2;

/// A parsed EAPOL-Key frame. `key_data` borrows the frame.
pub struct EapolKey<'a> {
    pub descriptor_type: u8,
    pub key_info: u16,
    pub key_length: u16,
    pub replay_counter: [u8; 8],
    pub nonce: [u8; 32],
    /// The Key RSC: for a group key, the packet number the AP last used with
    /// it, little-endian in the first six octets. Received group frames must
    /// carry a higher one.
    pub key_rsc: [u8; 8],
    pub mic: [u8; 16],
    pub key_data: &'a [u8],
}

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
