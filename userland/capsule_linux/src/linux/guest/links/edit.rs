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

/*
 * Removing and moving a symbolic link, for unlink and rename: a link made
 * by symlink lives in this table, not in the store.
 */

use alloc::vec::Vec;

use super::Links;

impl Links {
    /* Remove the link at `path`; false when there is none. */
    pub fn remove(&self, path: &[u8]) -> bool {
        let mut all = self.0.borrow_mut();
        let before = all.len();
        all.retain(|(from, _)| from != path);
        all.len() != before
    }

    /*
     * Move the link at `from` to `to`, replacing any there; false when
     * `from` is no link.
     */
    pub fn rename(&self, from: &[u8], to: Vec<u8>) -> bool {
        let Some(target) = self.target(from) else {
            return false;
        };
        let mut all = self.0.borrow_mut();
        all.retain(|(p, _)| p != from && p[..] != to[..]);
        all.push((to, target));
        true
    }
}
