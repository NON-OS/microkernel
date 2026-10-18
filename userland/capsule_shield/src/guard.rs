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

use nonos_vault::ROOT_LABEL;
use nox_shield_core::error::CustodyError;
use std::sync::OnceLock;

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
    pub(super) fn root(&self) -> Result<[u8; 32], CustodyError> {
        if self.live {
            return session();
        }
        nonos_libc::machine_key(ROOT_LABEL).map_err(|_| CustodyError::Guard)
    }
}

#[path = "guard_wrap.rs"]
mod guard_wrap;
