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

//! One SERVE_ONE frame to the core's endpoint and its answer, in the wire
//! (nonos_serve_wire, docs/design/serve-one.md).

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;
use nonos_serve_wire::{decode, put, Header, HEADER, MAX_BODY, OK};

use super::pipe::ServePipe;

/// How long any frame waits for its answer. A read asks for no wait of its
/// own, so this bounds every call the window makes.
pub(super) const CALL_WAIT_MS: u64 = 2000;

/// The answer's status, and its body.
pub(super) struct Answer {
    pub status: u8,
    pub conn: u16,
    pub body: Vec<u8>,
}

impl Answer {
    pub fn ok(&self) -> bool {
        self.status == OK
    }
}

impl ServePipe {
    pub(super) fn call(&self, op: u8, body: &[u8]) -> Option<Answer> {
        let mut ask = vec![0u8; HEADER + body.len()];
        let head = Header { op, status: OK, conn: self.conn, len: body.len() as u32 };
        let n = put(&mut ask, head, body)?;
        let mut got = vec![0u8; HEADER + MAX_BODY];
        let n = mk_ipc_call_timeout(
            self.port,
            ask.as_ptr(),
            n,
            got.as_mut_ptr(),
            got.len(),
            CALL_WAIT_MS,
        );
        let (h, body) = decode(got.get(..usize::try_from(n).ok()?)?).ok()?;
        (h.op == op).then(|| Answer { status: h.status, conn: h.conn, body: body.to_vec() })
    }
}
