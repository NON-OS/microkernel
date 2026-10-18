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

//! The file key sealed under the machine root, and opened again. The root,
//! the key and a failed opening are zeroed on every path.

use nonos_vault::{open, seal, OVERHEAD};
use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;
use zeroize::Zeroize;

use super::MachineGuard;

/// The record the file key is sealed under; no other capsule names it.
const RECORD: &[u8] = b"shield.store.file-key";

impl HardwareGuard for MachineGuard {
    fn wrap(&self, mut key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        let mut nonce = [0u8; 12];
        if nonos_libc::crypto_random(nonce.as_mut_ptr(), nonce.len()) != nonce.len() as i64 {
            key.zeroize();
            return Err(CustodyError::Entropy);
        }
        // The key is wiped on this path too, not only once it is sealed.
        let mut root = match self.root() {
            Ok(root) => root,
            Err(e) => {
                key.zeroize();
                return Err(e);
            }
        };
        let mut out = vec![0u8; key.len() + OVERHEAD];
        let sealed = seal(&root, RECORD, &key, &nonce, &mut out);
        root.zeroize();
        key.zeroize();
        sealed
            .map(|n| {
                out.truncate(n);
                out
            })
            .map_err(|_| CustodyError::Guard)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        let mut root = self.root()?;
        let mut out = vec![0u8; blob.len()];
        let opened = open(&root, RECORD, &blob, &mut out);
        root.zeroize();
        match opened {
            Ok(n) => {
                out.truncate(n);
                Ok(out)
            }
            Err(_) => {
                out.zeroize();
                Err(CustodyError::Guard)
            }
        }
    }
}
