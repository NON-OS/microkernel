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

//! The snapshot and the questions it answers.

/// Ceiling on one read. A response is bounded by the reply buffer long before
/// this, and the count the caller receives is what was actually walked.
pub const MAX_PROCS: usize = 128;

pub struct Capsule {
    pub name: [u8; 24],
    pub name_len: u8,
    pub caps: u64,
}

impl Capsule {
    pub fn name(&self) -> &[u8] {
        &self.name[..(self.name_len as usize).min(self.name.len())]
    }
}

pub struct Snapshot {
    pub capsules: [Capsule; MAX_PROCS],
    pub count: usize,
}

impl Snapshot {
    pub fn live(&self) -> &[Capsule] {
        &self.capsules[..self.count]
    }

    /// Holders of `mask` whose name `named` does not list.
    pub fn holders_beyond(&self, mask: u64, named: &[&[u8]]) -> u32 {
        let stray = |c: &&Capsule| c.caps & mask != 0 && !named.contains(&c.name());
        self.live().iter().filter(stray).count() as u32
    }

    /// Capsules holding any of `a` and also any of `b`.
    pub fn holders_both(&self, a: u64, b: u64) -> u32 {
        self.live().iter().filter(|c| c.caps & a != 0 && c.caps & b != 0).count() as u32
    }

    pub fn unmasked(&self) -> u32 {
        self.live().iter().filter(|c| c.caps == 0).count() as u32
    }
}
