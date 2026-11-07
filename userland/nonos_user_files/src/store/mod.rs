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

//! Object numbers and the committed generation.

mod checkpoint;
mod commit;
mod load;
mod read;

pub use commit::commit;
pub use load::load;
pub use read::read_full;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::errno::{Errno, ENOENT};
use crate::host::Objects;
use crate::manifest::Record;

/// The two slots alternate by generation: an odd one in object 1.
pub fn slot_of(generation: u64) -> u64 {
    generation % 2
}

pub struct Store {
    /// The newest generation on the volume; 0 before the first commit.
    pub generation: u64,
    /// The next object number. Numbers are handed out in order and never
    /// twice, so a cut save's objects are the ones at or past the
    /// committed manifest's `next_id` (load.rs).
    pub next_id: u64,
    /// The objects the committed body is in, its length and hash.
    pub parts: Vec<u64>,
    pub base_len: u64,
    pub base_hash: [u8; 32],
    /// The records committed since the body, and their bytes together.
    pub journal: Vec<Record>,
    pub journal_bytes: u64,
    /// Each committed node's print (manifest::print): a commit writes the
    /// nodes whose print changed, and names the ones gone.
    pub prints: BTreeMap<u64, [u8; 16]>,
    /// Objects nothing committed will need once the next manifest is in:
    /// written over, cut off, or a released file's. Each is removed after
    /// a commit, and named in that commit's body so a cut before the
    /// removal finishes it at the next start.
    pub pending: Vec<u64>,
}

impl Store {
    /// A volume nothing was committed to.
    pub fn empty(next_id: u64) -> Store {
        Store {
            generation: 0,
            next_id,
            parts: Vec::new(),
            base_len: 0,
            base_hash: [0; 32],
            journal: Vec::new(),
            journal_bytes: 0,
            prints: BTreeMap::new(),
            pending: Vec::new(),
        }
    }

    /// Writes `bytes` as the next object and only then takes its number,
    /// so a failed write leaves no gap for the start-up sweep to stop at.
    pub fn append(&mut self, objects: &mut impl Objects, bytes: &[u8]) -> Result<u64, Errno> {
        let id = self.next_id;
        objects.put(id, bytes)?;
        self.next_id += 1;
        Ok(id)
    }

    /// Removes every pending object it can; one the volume would not
    /// remove stays pending for the next commit to name again.
    pub fn remove_pending(&mut self, objects: &mut impl Objects) {
        let ids = core::mem::take(&mut self.pending);
        for id in ids {
            match objects.remove(id) {
                Ok(()) | Err(ENOENT) => {}
                Err(_) => self.pending.push(id),
            }
        }
    }
}
