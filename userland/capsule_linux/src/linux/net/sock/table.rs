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

//! The table itself: entries by index, a freed index reused.

use alloc::vec::Vec;

use super::types::{Domain, Proto, Sock};

pub struct Socks {
    pub(super) list: Vec<Option<Sock>>,
    /// Where the next ephemeral port search starts.
    pub(super) next_port: u16,
    /// Waiting calls partway through, by thread (`progress`).
    pub(super) progress: Vec<(u32, usize)>,
}

impl Socks {
    pub const fn new() -> Socks {
        Socks { list: Vec::new(), next_port: 0, progress: Vec::new() }
    }

    /// A fresh socket held by `pid`, or by nobody yet when `pid` is None (the
    /// server end of a connection, until accept hands it out).
    pub fn open(&mut self, domain: Domain, proto: Proto, pid: Option<u32>) -> u32 {
        let sock = Sock::new(domain, proto, pid);
        match self.list.iter().position(Option::is_none) {
            Some(i) => {
                self.list[i] = Some(sock);
                i as u32
            }
            None => {
                self.list.push(Some(sock));
                (self.list.len() - 1) as u32
            }
        }
    }

    pub fn get(&self, id: u32) -> Option<&Sock> {
        self.list.get(id as usize).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Sock> {
        self.list.get_mut(id as usize).and_then(Option::as_mut)
    }

    /// Every live socket, with its index.
    pub fn iter(&self) -> impl Iterator<Item = (u32, &Sock)> {
        self.list.iter().enumerate().filter_map(|(i, s)| s.as_ref().map(|s| (i as u32, s)))
    }
}
