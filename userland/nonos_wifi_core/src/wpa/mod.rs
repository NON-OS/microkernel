// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The WPA2/WPA3 security core: the key derivation the four-way handshake
//! rests on. SHA-1, HMAC-SHA1, PBKDF2 and the 802.11i PRF for WPA2-PSK;
//! HMAC-SHA256, the 802.11 KDF and HKDF for SAE and the SHA-256 AKMs; composed
//! into the pairwise master key and the pairwise transient key. Every primitive
//! is checked against RFC or IEEE known-answer vectors in the proof crates.

pub mod akm;
pub mod hkdf;
pub mod hmac;
pub mod kdf;
pub mod pbkdf2;
pub mod prf;
pub mod psk;
pub mod ptk;
mod rsn;
pub mod sha1;
pub mod sha256;
pub mod supplicant;

pub use rsn::RSN_IE;
