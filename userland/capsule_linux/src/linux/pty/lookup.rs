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

//! Finding a pseudo-terminal: whether /dev/pts/N opens, what /dev/pts
//! lists, and which one a session controls. Pure.

use alloc::vec::Vec;

use super::pair::Pair;
use super::table::Table;
use crate::linux::abi::errno;

impl Table {
    /// Whether /dev/pts/`n` opens: ENOENT with no such pair or its master
    /// gone, EIO while it is locked, as Linux's pts_unix98_lookup answers.
    pub fn may_open(&self, n: u32) -> Result<(), i64> {
        match self.pairs.get(n as usize).and_then(Option::as_ref) {
            Some(p) if p.hung => Err(errno::ENOENT),
            Some(p) if p.locked => Err(errno::EIO),
            Some(_) => Ok(()),
            None => Err(errno::ENOENT),
        }
    }

    /// The numbers /dev/pts lists: every pair whose master is open.
    pub fn names(&self) -> Vec<u32> {
        let live = |(_, p): &(usize, &Option<Pair>)| p.as_ref().is_some_and(|p| !p.hung);
        self.pairs.iter().enumerate().filter(live).map(|(n, _)| n as u32).collect()
    }

    /// The pair that is session `sid`'s controlling terminal.
    pub fn controlling(&self, sid: u32) -> Option<u32> {
        let ctl = |p: &Option<Pair>| p.as_ref().is_some_and(|p| sid != 0 && p.session == sid);
        self.pairs.iter().position(ctl).map(|n| n as u32)
    }
}
