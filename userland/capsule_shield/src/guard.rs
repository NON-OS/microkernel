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

//! The phones' hardware guard, on this machine: the store's file key is
//! wrapped under a key derived from the machine root, which the TPM gives
//! only to this machine in the boot state that sealed it.
//!
//! A machine booted live, or with no TPM, has no machine root. Its store is
//! kept in memory, and its file key is wrapped under a key drawn for this
//! boot and held only in this process: sealed all the same, and gone at
//! reboot with the store. A store on the data volume is never wrapped so.

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;
use nonos_vault::{open, seal, OVERHEAD, ROOT_LABEL};
use std::sync::OnceLock;

/// The record the file key is sealed under; no other capsule names it.
const RECORD: &[u8] = b"shield.store.file-key";

pub struct MachineGuard {
    /// The store is kept in memory, for this boot only.
    pub live: bool,
}

/// This boot's key for a store kept in memory, drawn once.
fn session() -> Result<[u8; 32], CustodyError> {
    static SESSION: OnceLock<Option<[u8; 32]>> = OnceLock::new();
    let drawn = SESSION.get_or_init(|| {
        let mut k = [0u8; 32];
        (nonos_libc::crypto_random(k.as_mut_ptr(), k.len()) == k.len() as i64).then_some(k)
    });
    drawn.ok_or(CustodyError::Entropy)
}

impl MachineGuard {
    fn root(&self) -> Result<[u8; 32], CustodyError> {
        if self.live {
            return session();
        }
        nonos_libc::machine_key(ROOT_LABEL).map_err(|_| CustodyError::Guard)
    }
}

fn wipe(b: &mut [u8]) {
    for x in b.iter_mut() {
        unsafe { core::ptr::write_volatile(x, 0) };
    }
}

impl HardwareGuard for MachineGuard {
    fn wrap(&self, mut key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        let mut nonce = [0u8; 12];
        if nonos_libc::crypto_random(nonce.as_mut_ptr(), nonce.len()) != nonce.len() as i64 {
            wipe(&mut key);
            return Err(CustodyError::Entropy);
        }
        let mut root = self.root()?;
        let mut out = vec![0u8; key.len() + OVERHEAD];
        let sealed = seal(&root, RECORD, &key, &nonce, &mut out);
        wipe(&mut root);
        wipe(&mut key);
        sealed.map(|n| {
            out.truncate(n);
            out
        })
        .map_err(|_| CustodyError::Guard)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        let mut root = self.root()?;
        let mut out = vec![0u8; blob.len()];
        let opened = open(&root, RECORD, &blob, &mut out);
        wipe(&mut root);
        match opened {
            Ok(n) => {
                out.truncate(n);
                Ok(out)
            }
            Err(_) => {
                wipe(&mut out);
                Err(CustodyError::Guard)
            }
        }
    }
}
