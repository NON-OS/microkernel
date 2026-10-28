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

//! The one SERVE_ONE connection to simplex.core: NSO1 messages on the
//! endpoint `linux.serve.app.simplex`, which only this capsule may call.

use alloc::vec::Vec;
use nonos_libc::mk_service_lookup;

use crate::link::nso1::{CLOSE, FRAME_MAX, OPEN, READ, WRITE};
use crate::link::pipe::Pipe;

use super::call::{CALL_WAIT_MS, READ_WAIT_MS};

const SERVICE: &[u8] = b"linux.serve.app.simplex";

pub struct ServePipe {
    pub(super) port: u64,
    pub(super) conn: u16,
}

impl ServePipe {
    /// The endpoint, and a connection on it; None while the core is not up.
    pub fn open() -> Option<Self> {
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return None;
        }
        let mut pipe = Self { port: u64::from(port), conn: 0 };
        let (head, _) = pipe.call(OPEN, &[], CALL_WAIT_MS)?;
        pipe.conn = head.conn;
        (head.op == OPEN).then_some(pipe)
    }
}

impl Pipe for ServePipe {
    fn read(&mut self) -> Option<Vec<u8>> {
        match self.call(READ, &[], READ_WAIT_MS) {
            Some((head, _)) if head.op == CLOSE => None,
            Some((_, body)) => Some(body),
            None => Some(Vec::new()),
        }
    }

    fn write(&mut self, bytes: &[u8]) -> bool {
        bytes
            .chunks(FRAME_MAX)
            .all(|c| self.call(WRITE, c, CALL_WAIT_MS).is_some_and(|(h, _)| h.op == WRITE))
    }
}

impl Drop for ServePipe {
    fn drop(&mut self) {
        let _ = self.call(CLOSE, &[], CALL_WAIT_MS);
    }
}
