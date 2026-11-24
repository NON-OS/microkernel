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

//! A front's grants, one row a personality, each bound to one family by the
//! kernel's generation for it. A request from a sender without Network is
//! served only while the sender's live family is the very family the grant
//! was made for, asked of the kernel on every request: a family sealed and
//! the next begun under the same pid, or a pid reused, is never served on an
//! earlier family's row.

use alloc::vec::Vec;

use super::{reaches, Facts, Grant, Refusal};

/// The most grants a front keeps; past this a new one is refused, never an
/// old one dropped to make room.
pub const MAX_ROWS: usize = 64;

struct Row {
    pid: u32,
    generation: u64,
    grant: Grant,
}

pub struct Grants {
    rows: Vec<Row>,
}

impl Grants {
    pub const fn new() -> Self {
        Self { rows: Vec::new() }
    }

    /// Keep `grant` for `pid`'s family `generation`, in place of any it had.
    pub fn insert(&mut self, pid: u32, generation: u64, grant: Grant) -> Result<(), Refusal> {
        self.forget(pid);
        if self.rows.len() >= MAX_ROWS {
            return Err(Refusal::Full);
        }
        self.rows.push(Row { pid, generation, grant });
        Ok(())
    }

    /// `pid` ended, or asked again: its row goes.
    pub fn forget(&mut self, pid: u32) {
        self.rows.retain(|r| r.pid != pid);
    }

    /// The grant `pid` is served on now, if its live family is the one it
    /// was made for. A row whose family has gone is dropped.
    fn current(&mut self, pid: u32, facts: &impl Facts) -> Option<&Grant> {
        let at = self.rows.iter().position(|r| r.pid == pid)?;
        if facts.generation(pid) != self.rows[at].generation {
            self.rows.swap_remove(at);
            return None;
        }
        Some(&self.rows[at].grant)
    }

    /// Whether a request from `pid` is served: a sender holding Network as
    /// always, any other only on its own family's grant.
    pub fn admits(&mut self, pid: u32, facts: &impl Facts) -> bool {
        facts.holds_network(pid) || self.current(pid, facts).is_some()
    }

    /// Whether a stream from `pid` may go to `host` on `port`: anywhere for
    /// a sender holding Network, as always; for any other, where its own
    /// family's grant lets it (reaches.rs).
    pub fn may_reach(&mut self, pid: u32, host: &[u8], port: u16, facts: &impl Facts) -> bool {
        if facts.holds_network(pid) {
            return true;
        }
        self.current(pid, facts).is_some_and(|g| reaches(g, host, port))
    }

    /// The pids holding a row.
    pub fn pids(&self) -> impl Iterator<Item = u32> + '_ {
        self.rows.iter().map(|r| r.pid)
    }

    /// Drop every row whose family has ended, as the front reaps its ended
    /// callers: a row nobody asks with again would otherwise hold its place.
    pub fn reap(&mut self, facts: &impl Facts) {
        self.rows.retain(|r| facts.generation(r.pid) == r.generation);
    }
}

impl Default for Grants {
    fn default() -> Self {
        Self::new()
    }
}
