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

//! Writing a reply.

use alloc::vec::Vec;

use super::header::{Header, HEADER};
use crate::errno::Errno;
use crate::node::{Attr, Kind};

/// ino u64, kind u8, mode u32, size u64, mtime u64, ctime u64, nlink u32.
pub const ATTR_LEN: usize = 41;

pub struct Reply(pub Vec<u8>);

impl Reply {
    /// A reply to `op`/`id` with status 0, its body to follow.
    pub fn ok(op: u16, id: u32) -> Reply {
        let mut out = Vec::with_capacity(64);
        out.resize(HEADER, 0);
        out.extend_from_slice(&0i32.to_le_bytes());
        let mut r = Reply(out);
        r.head(op, id);
        r
    }

    /// A reply with no body and the negated `errno`.
    pub fn fail(op: u16, id: u32, errno: Errno) -> Reply {
        let mut r = Reply::ok(op, id);
        r.0[HEADER..HEADER + 4].copy_from_slice(&(-errno).to_le_bytes());
        r
    }

    fn head(&mut self, op: u16, id: u32) {
        let body_len = (self.0.len() - HEADER) as u32;
        let mut head = [0u8; HEADER];
        Header { op, id, body_len }.write(&mut head);
        self.0[..HEADER].copy_from_slice(&head);
    }

    /// Appends to the body and keeps the header's length true.
    pub fn put(&mut self, bytes: &[u8]) -> &mut Reply {
        self.0.extend_from_slice(bytes);
        let len = ((self.0.len() - HEADER) as u32).to_le_bytes();
        self.0[12..16].copy_from_slice(&len);
        self
    }
}

pub fn attr_bytes(a: &Attr) -> [u8; ATTR_LEN] {
    let mut out = [0u8; ATTR_LEN];
    out[0..8].copy_from_slice(&a.ino.to_le_bytes());
    out[8] = kind_byte(a.kind);
    out[9..13].copy_from_slice(&a.mode.to_le_bytes());
    out[13..21].copy_from_slice(&a.size.to_le_bytes());
    out[21..29].copy_from_slice(&a.mtime.to_le_bytes());
    out[29..37].copy_from_slice(&a.ctime.to_le_bytes());
    out[37..41].copy_from_slice(&a.nlink.to_le_bytes());
    out
}

pub fn kind_byte(kind: Kind) -> u8 {
    match kind {
        Kind::Dir => 0,
        Kind::File => 1,
    }
}
