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

//! What LISTING has read of each chain's registry so far (v1.4.16): the
//! last block scanned and every row from deploy_block to it, so a later
//! call reads only new blocks. Bounded: once the rows would pass MAX_ROWS
//! the cursor gives up its cache and each call rescans for its own listing.

use alloc::vec::Vec;

use super::scan::Found;

pub const MAX_ROWS: usize = 1024;

pub struct Cursor {
    pub chain_id: u64,
    /// The last block whose logs are in `found`; 0 before the first scan.
    pub scanned: u64,
    pub found: Found,
    /// The cache passed MAX_ROWS: rescan per listing from now on.
    pub full: bool,
}

#[derive(Default)]
pub struct Cursors(Vec<Cursor>);

impl Cursors {
    /// The chain's cursor, made fresh on the chain's first ask.
    pub fn get(&mut self, chain_id: u64) -> Option<&mut Cursor> {
        let at = match self.0.iter().position(|c| c.chain_id == chain_id) {
            Some(at) => at,
            None => {
                let at = self.0.len();
                self.0.push(Cursor { chain_id, scanned: 0, found: Found::default(), full: false });
                at
            }
        };
        self.0.get_mut(at)
    }
}

impl Cursor {
    /// Take in a scan of the blocks after `scanned` up to `head`.
    pub fn absorb(&mut self, more: Found, head: u64) {
        if self.found.rows().saturating_add(more.rows()) > MAX_ROWS {
            self.full = true;
            self.found = Found::default();
            return;
        }
        self.found.published.extend(more.published);
        self.found.withdrawn.extend(more.withdrawn);
        self.scanned = self.scanned.max(head);
    }
}
