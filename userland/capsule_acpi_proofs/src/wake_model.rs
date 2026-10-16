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

//! A model of the kernel's claim test for the wake sweep: which pid holds
//! each device id and what it resolves to, with the three ways a claim
//! ends. Four devices armed by three pids.

use crate::wake_sweep::{stale, Owner};
use std::collections::BTreeMap;

/// device id -> (namespace id, holder pid); departed devices are gone.
#[derive(Clone, Default)]
pub struct Kernel(pub BTreeMap<u64, (u32, Option<u32>)>);

impl Kernel {
    pub fn holds(&self, pid: u32, id: u64) -> Option<u32> {
        self.0.get(&id).filter(|(_, h)| *h == Some(pid)).map(|(dev, _)| *dev)
    }
    pub fn release(&mut self, id: u64) {
        if let Some(e) = self.0.get_mut(&id) {
            e.1 = None;
        }
    }
    pub fn exit(&mut self, pid: u32) {
        self.0.values_mut().filter(|e| e.1 == Some(pid)).for_each(|e| e.1 = None);
    }
    pub fn depart(&mut self, id: u64) {
        self.0.remove(&id);
    }
}

pub fn armed() -> (Kernel, Vec<Owner>) {
    let mut k = Kernel::default();
    for (id, dev, pid) in [(10, 1, 100), (11, 2, 100), (12, 3, 200), (13, 4, 300)] {
        k.0.insert(id, (dev, Some(pid)));
    }
    (k, vec![(1, 100, 10), (2, 100, 11), (3, 200, 12), (4, 300, 13)])
}

pub fn swept(k: &Kernel, owners: &[Owner]) -> Vec<u32> {
    stale(owners, &mut |pid, id| k.holds(pid, id)).iter().map(|o| o.0).collect()
}
