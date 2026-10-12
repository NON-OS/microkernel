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

//! Keeping a new bond and forgetting one. The new record is stored before
//! the old one is deleted, so a refusal part way leaves the old bond, not
//! none.

use super::types::Store;
use crate::error::BondError;
use crate::keys::{Keys, KeysError, KIND_BOND};
use crate::record::BondRecord;
use crate::wipe::wipe;
use crate::MAX_BONDS;

impl<K: Keys> Store<K> {
    pub fn keep(&mut self, record: BondRecord) -> Result<(), BondError> {
        let (kind, identity) = (record.identity_type, record.identity);
        let old = self.id_of(kind, &identity);
        if old.is_none() && self.book.bonds().len() >= MAX_BONDS {
            return Err(BondError::Full);
        }
        let mut bytes = record.encode()?;
        let stored = self.keys.store(KIND_BOND, &bytes);
        wipe(&mut bytes);
        let id = stored?;
        if let Some(at) = old {
            match self.keys.delete(self.ids[at].2) {
                Ok(()) | Err(KeysError::NotFound) => drop(self.ids.remove(at)),
                Err(e) => {
                    // The new record goes, so the old bond stays the one
                    // kept. Should that delete fail too, the first error
                    // is the one that says why; both records then remain.
                    let _ = self.keys.delete(id);
                    return Err(e.into());
                }
            }
        }
        self.ids.push((kind, identity, id));
        self.book.insert(record)
    }

    /// Forget the bond with this identity; whether there was one.
    pub fn forget(&mut self, kind: u8, identity: &[u8; 6]) -> Result<bool, BondError> {
        let Some(at) = self.id_of(kind, identity) else { return Ok(false) };
        self.keys.delete(self.ids[at].2)?;
        self.ids.remove(at);
        Ok(self.book.remove(kind, identity))
    }
}
