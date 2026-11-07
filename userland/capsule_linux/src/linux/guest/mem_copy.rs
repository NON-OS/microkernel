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

//! Bytes in and out of a guest, a megabyte at a time.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::peer::{mk_peer_read, mk_peer_read_rows, mk_peer_write};

use super::handle::Guest;
use super::mem::MAX_SPAN;
use super::mem_rows;
use super::memory::Memory;

impl Memory for Guest {
    fn read_at(&self, at: u64, len: usize) -> Option<Vec<u8>> {
        self.read(at, len)
    }

    fn write_at(&self, at: u64, bytes: &[u8]) -> i64 {
        self.write(at, bytes)
    }
}

impl Guest {
    /// Bytes written, or the first failure.
    pub fn write(&self, addr: u64, bytes: &[u8]) -> i64 {
        let mut done = 0usize;
        while done < bytes.len() {
            let take = (bytes.len() - done).min(MAX_SPAN as usize);
            let rc = mk_peer_write(self.pid, addr + done as u64, &bytes[done..done + take]);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        bytes.len() as i64
    }

    pub fn read(&self, addr: u64, len: usize) -> Option<Vec<u8>> {
        let mut out = vec![0u8; len];
        Guest::read_into(self.pid, addr, &mut out).then_some(out)
    }

    /// Fill `out` from guest `pid` at `addr`, false on the first failure: a
    /// read into a buffer this capsule already holds, so a large one is not
    /// held twice.
    pub fn read_into(pid: u32, addr: u64, out: &mut [u8]) -> bool {
        let mut done = 0usize;
        while done < out.len() {
            let take = (out.len() - done).min(MAX_SPAN as usize);
            if mk_peer_read(pid, addr + done as u64, &mut out[done..done + take]) < 0 {
                return false;
            }
            done += take;
        }
        true
    }

    /// Fill `count` rows of `row` bytes of `out` from guest `pid`, row `i`
    /// from `addr + i * stride` to `out[i * stride..]`, false on the first
    /// failure; the bytes of `out` between the rows are left as they are.
    /// As few kernel calls as the rows fit (mem_rows.rs), and a row wider
    /// than one call is read as its own span.
    pub fn read_rows_into(
        pid: u32,
        addr: u64,
        out: &mut [u8],
        (row, count, stride): (usize, usize, usize),
    ) -> bool {
        let Some(calls) = mem_rows::calls(row, count) else {
            return (0..count).all(|i| Guest::read_row(pid, addr, out, i, (row, stride)));
        };
        for (first, n) in calls {
            let Some(at) = first.checked_mul(stride) else { return false };
            let (Some(from), Some(part)) = (addr.checked_add(at as u64), out.get_mut(at..)) else {
                return false;
            };
            if mk_peer_read_rows(pid, from, part, row, n, stride) < 0 {
                return false;
            }
        }
        true
    }

    /// Row `i` of `row` bytes, `stride` apart, read alone into `out`.
    fn read_row(
        pid: u32,
        addr: u64,
        out: &mut [u8],
        i: usize,
        (row, stride): (usize, usize),
    ) -> bool {
        let Some(at) = i.checked_mul(stride) else { return false };
        let (Some(from), Some(end)) = (addr.checked_add(at as u64), at.checked_add(row)) else {
            return false;
        };
        out.get_mut(at..end).is_some_and(|px| Guest::read_into(pid, from, px))
    }
}
