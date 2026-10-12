// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//! The WPA2 key derivation: the pairwise master key from the passphrase and
//! SSID, and the pairwise transient key from the master key, the two MAC
//! addresses and the two nonces of the four-way handshake. This is the secret
//! the whole link rests on. Checked against the IEEE 802.11i PMK vectors.

use super::pbkdf2::pbkdf2_sha1;
pub use super::ptk_into::{ptk_into, ptk_sha256_into};

/// The pairwise master key: PBKDF2(passphrase, ssid, 4096, 32).
pub fn pmk(passphrase: &[u8], ssid: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    pbkdf2_sha1(passphrase, ssid, 4096, &mut out);
    out
}

/// The 384-bit pairwise transient key of a CCMP-128 link (`ptk_into` with
/// 48 octets).
pub fn ptk(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
) -> [u8; 48] {
    let mut out = [0u8; 48];
    ptk_into(pmk, aa, spa, anonce, snonce, &mut out);
    out
}

/// The 384-bit pairwise transient key of a CCMP-128 link for the SHA-256
/// AKMs (`ptk_sha256_into` with 48 octets).
pub fn ptk_sha256(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
) -> [u8; 48] {
    let mut out = [0u8; 48];
    // 384 bits fits the KDF's 16-bit length field, so this never refuses.
    let _ = ptk_sha256_into(pmk, aa, spa, anonce, snonce, &mut out);
    out
}
