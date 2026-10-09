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

//! Which exit to try next, from what this session has seen of them.
//!
//! Rotation used to take the next exit in the directory's list, whatever had
//! happened with it before: with most published exits silent for a while, a
//! session went round the same silent ones again. Now the session keeps the
//! identities of exits that went silent on it (at most SILENT_MAX, the
//! oldest forgotten first) and of exits that delivered (at most PROVEN_MAX),
//! and the next exit is the first of the next LOOK_MAX in the directory not
//! seen silent; failing that, one that delivered earlier in the session;
//! failing that, the one that went silent longest ago.
//!
//! Pure and bounded; capsule_socks5_proofs holds it.

extern crate alloc;

use alloc::vec::Vec;

/// Silent exits remembered for the session.
pub const SILENT_MAX: usize = 16;

/// Exits that delivered, remembered for the session.
pub const PROVEN_MAX: usize = 4;

/// Directory places looked at for one pick.
pub const LOOK_MAX: u32 = 8;

/// What an exit is known by.
pub trait Named {
    fn id(&self) -> [u8; 32];
}

/// What this session has seen of exits.
pub struct Record<T> {
    /// Oldest first.
    silent: Vec<[u8; 32]>,
    /// Oldest first.
    proven: Vec<T>,
}

impl<T> Default for Record<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Record<T> {
    pub const fn new() -> Self {
        Record { silent: Vec::new(), proven: Vec::new() }
    }
}

impl<T: Named + Copy> Record<T> {
    /// `exit` used up its silence budget.
    pub fn silent(&mut self, exit: &T) {
        let id = exit.id();
        self.silent.retain(|s| *s != id);
        self.silent.push(id);
        if self.silent.len() > SILENT_MAX {
            self.silent.remove(0);
        }
    }

    /// Something came back through `exit`: it works, so it is not silent.
    pub fn proven(&mut self, exit: T) {
        let id = exit.id();
        self.silent.retain(|s| *s != id);
        self.proven.retain(|p| p.id() != id);
        self.proven.push(exit);
        if self.proven.len() > PROVEN_MAX {
            self.proven.remove(0);
        }
    }

    pub fn is_silent(&self, exit: &T) -> bool {
        self.silent.contains(&exit.id())
    }

    /// The exit to try, and the directory place it was taken from, looking
    /// at places `start` onward through `at`.
    pub fn pick(&self, start: u32, mut at: impl FnMut(u32) -> Option<T>) -> Option<(T, u32)> {
        let mut seen: Vec<(T, u32)> = Vec::new();
        for place in start..start.saturating_add(LOOK_MAX) {
            let Some(exit) = at(place) else { break };
            if !self.is_silent(&exit) {
                return Some((exit, place));
            }
            seen.push((exit, place));
        }
        if let Some(&exit) = self.proven.last() {
            return Some((exit, start));
        }
        /* Every one looked at went silent: the one that did longest ago. */
        let age = |e: &T| self.silent.iter().position(|s| *s == e.id()).unwrap_or(usize::MAX);
        seen.into_iter().min_by_key(|(e, _)| age(e))
    }
}
