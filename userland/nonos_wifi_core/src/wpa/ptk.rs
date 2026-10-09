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
use super::prf::prf;

/// The pairwise master key: PBKDF2(passphrase, ssid, 4096, 32).
pub fn pmk(passphrase: &[u8], ssid: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    pbkdf2_sha1(passphrase, ssid, 4096, &mut out);
    out
}

/// The 384-bit pairwise transient key. The MAC addresses and nonces are ordered
/// (smaller first) so both sides derive the same key regardless of who is the
/// authenticator.
pub fn ptk(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
) -> [u8; 48] {
    let data = ordered_context(aa, spa, anonce, snonce);
    let mut out = [0u8; 48];
    prf(pmk, b"Pairwise key expansion", &data, &mut out);
    out
}

/// The 384-bit pairwise transient key for the SHA-256 AKMs (SAE, PSK-SHA256):
/// KDF-SHA256-384(PMK, "Pairwise key expansion", the same ordered addresses
/// and nonces), IEEE Std 802.11-2020, 12.7.1.3. Only the expansion differs
/// from `ptk`; the KCK, KEK and TK are again the three 16-byte thirds.
pub fn ptk_sha256(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
) -> [u8; 48] {
    let data = ordered_context(aa, spa, anonce, snonce);
    let mut out = [0u8; 48];
    // 384 bits fits the KDF's 16-bit length field, so this never refuses.
    let _ = super::kdf::kdf_sha256(pmk, b"Pairwise key expansion", &data, &mut out);
    out
}

// Min(AA,SPA) || Max(AA,SPA) || Min(ANonce,SNonce) || Max(ANonce,SNonce).
fn ordered_context(aa: &[u8; 6], spa: &[u8; 6], anonce: &[u8; 32], snonce: &[u8; 32]) -> [u8; 76] {
    let (a1, a2) = if aa <= spa { (aa, spa) } else { (spa, aa) };
    let (n1, n2) = if anonce <= snonce { (anonce, snonce) } else { (snonce, anonce) };
    let mut data = [0u8; 76];
    data[0..6].copy_from_slice(a1);
    data[6..12].copy_from_slice(a2);
    data[12..44].copy_from_slice(n1);
    data[44..76].copy_from_slice(n2);
    data
}
