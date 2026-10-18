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

//! This host's IRK: read back if one is kept, else drawn and kept, so the
//! private addresses of every boot resolve under the key bonded peers hold
//! (Vol 3, Part C, 10.7.1).

use super::types::Store;
use crate::error::BondError;
use crate::keys::{Keys, KIND_LOCAL_IRK};
use crate::local::LocalIrk;
use crate::wipe::wipe;

impl<K: Keys> Store<K> {
    /// The kept key, or one from `draw` kept now. `draw` is called only
    /// when none is kept; a zero draw is refused, as no draw took place.
    pub fn local_irk(&mut self, draw: impl FnOnce() -> [u8; 16]) -> Result<LocalIrk, BondError> {
        for id in self.keys.list(KIND_LOCAL_IRK)? {
            let mut bytes = self.keys.retrieve(id)?;
            let read = LocalIrk::decode(&bytes);
            wipe(&mut bytes);
            if let Ok(irk) = read {
                return Ok(irk);
            }
        }
        let mut drawn = draw();
        let irk = LocalIrk::new(drawn);
        wipe(&mut drawn);
        let irk = irk?;
        let mut bytes = irk.encode();
        let stored = self.keys.store(KIND_LOCAL_IRK, &bytes);
        wipe(&mut bytes);
        stored?;
        Ok(irk)
    }
}
