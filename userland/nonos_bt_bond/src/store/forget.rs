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

//! Forgetting a bond. Every keyring record for the identity is deleted, not
//! only the one the book tracks: a `keep` whose cleanup the keyring refused
//! may have left a stray copy, which would otherwise come back on the next
//! `open` — at whatever level it held, raising a peer the owner forgot back
//! to trusted. So forget sweeps the keyring by identity, then the book.

use super::types::Store;
use crate::error::BondError;
use crate::keys::{Keys, KIND_BOND};
use crate::record::BondRecord;
use crate::wipe::wipe;

impl<K: Keys> Store<K> {
    /// Forget the bond with this identity; whether the book held one.
    pub fn forget(&mut self, kind: u8, identity: &[u8; 6]) -> Result<bool, BondError> {
        for id in self.keys.list(KIND_BOND)? {
            let mut bytes = self.keys.retrieve(id)?;
            let mine = BondRecord::decode(&bytes)
                .map(|r| r.identity_type == kind && &r.identity == identity)
                .unwrap_or(false);
            wipe(&mut bytes);
            if mine {
                self.keys.delete(id)?;
            }
        }
        self.ids.retain(|(k, id, _)| !(*k == kind && id == identity));
        Ok(self.book.remove(kind, identity))
    }
}
