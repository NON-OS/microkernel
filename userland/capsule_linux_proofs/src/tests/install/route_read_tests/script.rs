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

//! A scripted far end, and the reply it sends in pieces.

use alloc::vec::Vec;

use crate::install::route_read::Source;

/// A stream that hands out `pieces` in order, then zeros, and records the
/// wait each read was given.
pub(super) struct Script {
    pub(super) pieces: Vec<Result<Vec<u8>, &'static str>>,
    pub(super) next: usize,
    pub(super) ends_after_last: bool,
    pub(super) waits: Vec<u64>,
}

impl Script {
    pub(super) fn new(pieces: Vec<Result<Vec<u8>, &'static str>>) -> Script {
        Script { pieces, next: 0, ends_after_last: false, waits: Vec::new() }
    }
}

impl Source for Script {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.waits.push(wait_ms);
        let Some(piece) = self.pieces.get(self.next) else { return Ok(0) };
        self.next += 1;
        let bytes = piece.clone()?;
        assert!(bytes.len() <= into.len(), "a read asks for at most CHUNK bytes");
        into[..bytes.len()].copy_from_slice(&bytes);
        Ok(bytes.len())
    }

    fn ended(&self) -> bool {
        self.ends_after_last && self.next >= self.pieces.len()
    }
}

pub(super) const REPLY: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nhello world";

pub(super) fn split(at: &[usize]) -> Vec<Result<Vec<u8>, &'static str>> {
    let mut out = Vec::new();
    let mut from = 0;
    for &to in at.iter().chain(core::iter::once(&REPLY.len())) {
        out.push(Ok(REPLY[from..to].to_vec()));
        from = to;
    }
    out
}
