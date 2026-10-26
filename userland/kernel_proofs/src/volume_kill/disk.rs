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

//! The disk the model writes: every block written, with the blocks it
//! points at, and the header ring. A directory node D whose first record R0
//! names a file node N; N's index is what a close replaces.

use std::collections::{BTreeMap, BTreeSet};

use super::kinds::{Dir, Write};

pub(super) const D: u64 = 1;
pub(super) const R0: u64 = 2;
pub(super) const N: u64 = 3;
pub(super) const FIRST_FREE: u64 = 4;

#[derive(Clone, Debug)]
pub(super) struct Disk {
    pub(super) blocks: BTreeMap<u64, BTreeSet<u64>>,
    pub(super) recs: BTreeSet<u64>,
    pub(super) headers: Vec<(u64, u64)>,
}

impl Disk {
    pub(super) fn new(dir: Dir) -> Disk {
        let mut blocks = BTreeMap::from([(D, BTreeSet::new()), (N, BTreeSet::new())]);
        let mut recs = BTreeSet::new();
        if dir != Dir::NoRecord {
            blocks.insert(D, BTreeSet::from([R0]));
            blocks.insert(R0, BTreeSet::from([N]));
            recs.insert(R0);
        }
        Disk { blocks, recs, headers: vec![(1, FIRST_FREE)] }
    }

    pub(super) fn land(&mut self, w: &Write) {
        match w {
            Write::Header { generation, free } => self.headers.push((*generation, *free)),
            Write::Block { lba, to, rec } => {
                self.blocks.insert(*lba, to.clone());
                if *rec {
                    self.recs.insert(*lba);
                }
            }
        }
    }

    pub(super) fn points(&self, lba: u64) -> BTreeSet<u64> {
        self.blocks.get(&lba).cloned().unwrap_or_default()
    }

    /// The free count the newest header on the disk gives a reboot.
    pub(super) fn reboot_free(&self) -> u64 {
        self.headers.iter().max_by_key(|h| h.0).expect("a header").1
    }
}
