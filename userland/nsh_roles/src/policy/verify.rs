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

//! Trusting a role table. The table is signed by a role authority it lists,
//! and that authority must be the one key id the host pins. Both halves of
//! the signature must verify over `H("nsh/1 policy\0" || signed bytes)`.

use crate::hybrid::verify;
use crate::policy::{Table, Unverified};
use crate::PolicyError;
use core::ops::Deref;
use nsh_crypto::{labelled, Label};
use nsh_wire::keys::KEY_ID_LEN;

/// A table whose signature verified under the pinned key. Only `verify`
/// makes one, so a check can never run on a table nobody signed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy<'a>(Table<'a>);

impl<'a> Deref for Policy<'a> {
    type Target = Table<'a>;

    fn deref(&self) -> &Table<'a> {
        &self.0
    }
}

impl<'a> Unverified<'a> {
    /// `min_serial` is the highest serial this host has installed. A lower
    /// one is an older table replayed; the same one may be reloaded.
    pub fn verify(
        self,
        anchor: &[u8; KEY_ID_LEN],
        min_serial: u64,
    ) -> Result<Policy<'a>, PolicyError> {
        if self.signer != anchor {
            return Err(PolicyError::Signer);
        }
        let key = self.table.authority(self.signer).ok_or(PolicyError::Signer)?;
        if !verify(&key.public, &labelled(Label::Policy, self.signed), &self.sig) {
            return Err(PolicyError::Signature);
        }
        if self.table.serial < min_serial {
            return Err(PolicyError::Replayed);
        }
        Ok(Policy(self.table))
    }
}

/// Read a table and verify it under the pinned key id, in one step.
pub fn open_policy<'a>(
    body: &'a [u8],
    anchor: &[u8; 32],
    min_serial: u64,
) -> Result<Policy<'a>, PolicyError> {
    Unverified::parse(body)?.verify(anchor, min_serial)
}
