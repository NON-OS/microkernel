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
//! The shield's record: an HD wallet's recovery words, as a count and 24
//! indices, sealed.
//!
//! Its own record name, so its key is derived apart from the account key's
//! and neither record opens as the other.

use nonos_vault::{open as vault_open, seal as vault_seal, OVERHEAD};

use super::error::VaultError;
use super::root::machine_root;
use super::wipe::wipe32;

const RECORD: &[u8] = b"keyring.wallet.shield";

/// The count, then 24 indices of two bytes each.
pub const SEED_LEN: usize = 1 + 2 * 24;
pub const SEED_BLOB_LEN: usize = SEED_LEN + OVERHEAD;

pub fn seal_seed(seed: &[u8; SEED_LEN]) -> Result<[u8; SEED_BLOB_LEN], VaultError> {
    let mut draw = [0u8; 32];
    if !crate::entropy::gather_secret(&mut draw) {
        return Err(VaultError::NoEntropy);
    }
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&draw[..12]);
    wipe32(&mut draw);
    let mut root = machine_root().map_err(|_| VaultError::NoKey)?;
    let mut out = [0u8; SEED_BLOB_LEN];
    let sealed = vault_seal(&root, RECORD, seed, &nonce, &mut out);
    wipe32(&mut root);
    sealed.map(|_| out).map_err(VaultError::from)
}

pub fn open_seed(bytes: &[u8]) -> Result<[u8; SEED_LEN], VaultError> {
    let mut root = machine_root().map_err(|_| VaultError::NoKey)?;
    let mut seed = [0u8; SEED_LEN];
    let opened = vault_open(&root, RECORD, bytes, &mut seed);
    wipe32(&mut root);
    match opened {
        Ok(SEED_LEN) => Ok(seed),
        other => {
            for b in seed.iter_mut() {
                unsafe { core::ptr::write_volatile(b, 0) };
            }
            Err(match other {
                Err(e) => VaultError::from(e),
                Ok(_) => VaultError::from(nonos_vault::VaultError::BadLength),
            })
        }
    }
}
