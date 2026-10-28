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

//! The model's write and cut of a file, each shown to every map of it as
//! echo.rs shows it.

use super::model::{World, PATHS};
use super::support::Entry;
use crate::linux::file::shared_maps::spans::{echo, zeroed};

fn put(v: &mut Vec<u8>, at: usize, bytes: &[u8]) {
    v.resize(v.len().max(at + bytes.len()), 0);
    v[at..at + bytes.len()].copy_from_slice(bytes);
}

impl World {
    pub(super) fn showing(&self, i: usize) -> Vec<Entry> {
        self.table.iter().filter(|e| e.shows(PATHS[i])).cloned().collect()
    }

    /// seen.rs's write: a gap past the end zeroed in the maps, then echo.
    pub fn write(&mut self, i: usize, at: u64, bytes: &[u8]) {
        let was = self.copy[i].len() as u64;
        if at > was {
            self.zero(i, was, at);
        }
        put(&mut self.truth[i], at as usize, bytes);
        put(&mut self.copy[i], at as usize, bytes);
        for e in self.showing(i) {
            let Some((addr, from, to)) = echo(&e, at, bytes.len() as u64) else { continue };
            self.mem[e.pid as usize][addr as usize..][..to - from]
                .copy_from_slice(&bytes[from..to]);
        }
    }

    pub fn truncate(&mut self, i: usize, len: u64) {
        let was = self.copy[i].len() as u64;
        self.truth[i].resize(len as usize, 0);
        self.copy[i].resize(len as usize, 0);
        self.zero(i, was, len);
    }

    fn zero(&mut self, i: usize, was: u64, len: u64) {
        for e in self.showing(i) {
            let Some((addr, n)) = zeroed(&e, was, len) else { continue };
            self.mem[e.pid as usize][addr as usize..][..n as usize].fill(0);
        }
    }
}
