// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What `nsh` reads: the user's key, in nsh-enrol's secret file format
//! (`NSHA`, version 1, the Ed25519 seed, the ML-DSA-65 public then secret
//! key); the fleet's authority keys, 1984 bytes each; withdrawn
//! enrolments, 64-byte digests end to end; and the accepted releases, one a
//! line:
//!
//! ```text
//! release VERSION PCR8 PCR9 LOADER FRONT NETWORK,...|-
//! ```

pub use super::releases::releases;
use crate::{HybridKey, UserKey};
use alloc::string::String;
use alloc::vec::Vec;
use nonos_mldsa65::{SecretKey, PUBLIC_LEN, SECRET_LEN};
use nsh_crypto::Secret;

/// A new file readable by its owner alone, written whole; refused if
/// anything, a link included, already has the name.
pub fn write_private(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(path)?;
    f.write_all(bytes)
}

pub fn read(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| alloc::format!("{path}: {e}"))
}

pub fn user_key(path: &str) -> Result<UserKey, String> {
    let mut b = read(path)?;
    let bad = || alloc::format!("{path}: not an nsh key file");
    if b.len() != 8 + 32 + PUBLIC_LEN + SECRET_LEN || &b[..8] != b"NSHA\x01\x00\x00\x00" {
        return Err(bad());
    }
    let mut seed: [u8; 32] = b[8..40].try_into().map_err(|_| bad())?;
    let public: [u8; PUBLIC_LEN] = b[40..40 + PUBLIC_LEN].try_into().map_err(|_| bad())?;
    let sk: &[u8; SECRET_LEN] = b[40 + PUBLIC_LEN..].try_into().map_err(|_| bad())?;
    let key = UserKey::from_parts(Secret::take(&mut seed), SecretKey::from_bytes(sk), &public);
    nonos_hash::wipe(&mut b);
    Ok(key)
}

pub fn authority(path: &str) -> Result<HybridKey, String> {
    HybridKey::from_bytes(&read(path)?).ok_or_else(|| alloc::format!("{path}: not a 1984-byte key"))
}

pub fn revoked(path: &str) -> Result<Vec<[u8; 64]>, String> {
    let b = read(path)?;
    if b.is_empty() || b.len() % 64 != 0 {
        return Err(alloc::format!("{path}: not a list of 64-byte digests"));
    }
    Ok(b.chunks_exact(64).filter_map(|d| d.try_into().ok()).collect())
}
