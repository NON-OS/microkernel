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

//! The model's write-backs, as back.rs makes them, and the calls that make
//! them: a read, a flush and a munmap.

use super::model::{ix, World};
use super::support::{Entry, P};
use crate::linux::file::shared_maps::rules::unmap;
use crate::linux::file::shared_maps::spans::{back_range, differing, going};

/// Compared a piece at a time, as back.rs does, but smaller.
const PIECE: u64 = 2 * P;

impl World {
    /// What writer `e` changed of the file bytes `want`, into the copy.
    pub fn back(&mut self, e: &Entry, want: (u64, u64)) {
        let copy = &mut self.copy[ix(&e.path)];
        let range = back_range(e, want, copy.len() as u64).filter(|_| e.writer && !e.orphan);
        let Some((mut at, hi)) = range else { return };
        while at < hi {
            let n = (hi - at).min(PIECE) as usize;
            let map = &self.mem[e.pid as usize][e.addr(at) as usize..][..n];
            let file = copy[at as usize..][..n].to_vec();
            for (a, b) in differing(map, &file) {
                copy[at as usize + a..at as usize + b].copy_from_slice(&map[a..b]);
            }
            at += n as u64;
        }
    }

    /// read: what writers changed of `[at, at + len)` first, then the copy.
    pub fn read(&mut self, i: usize, at: u64, len: u64) -> Vec<u8> {
        for e in self.showing(i) {
            self.back(&e, (at, at + len));
        }
        let end = self.copy[i].len();
        self.copy[i][(at as usize).min(end)..((at + len) as usize).min(end)].to_vec()
    }

    pub fn flush(&mut self, i: usize) {
        self.showing(i).iter().for_each(|e| self.back(e, e.file()));
    }

    pub fn unmap(&mut self, pid: u32, at: u64, span: u64) -> Result<(), i64> {
        let kept = unmap(&self.table, pid, at, span)?;
        for e in self.table.clone().iter().filter(|e| e.pid == pid) {
            let Some(part) = going(e, at, span) else { continue };
            self.back(e, part);
        }
        self.table = kept;
        Ok(())
    }
}
