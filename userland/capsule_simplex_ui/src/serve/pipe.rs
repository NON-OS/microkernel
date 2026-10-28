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

//! The one SERVE_ONE connection to simplex.core, on an endpoint only this
//! capsule may call. A read asks with no wait, so a tick never blocks; a
//! write is offered again, a bounded number of times, while the queue is full.

use alloc::vec::Vec;
use nonos_libc::{mk_idle_ms, mk_service_lookup};
use nonos_serve_wire::{read_ask, took_of, CLOSE, MAX_BODY, OPEN, READ, WRITE};

use crate::link::pipe::Pipe;

const SERVICE: &[u8] = b"linux.serve.app.simplex";
const FULL_TRIES: u32 = 200;

pub struct ServePipe {
    pub(super) port: u64,
    pub(super) conn: u16,
}

impl ServePipe {
    /// A connection; None while the endpoint or the core's listen is not up.
    pub fn open() -> Option<Self> {
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return None;
        }
        let pipe = Self { port: u64::from(port), conn: 0 };
        let answer = pipe.call(OPEN, &[])?;
        answer.ok().then_some(Self { conn: answer.conn, ..pipe })
    }
}

impl Pipe for ServePipe {
    fn read(&mut self) -> Option<Vec<u8>> {
        let a = self.call(READ, &read_ask(MAX_BODY as u32, 0))?;
        a.ok().then_some(a.body)
    }

    fn write(&mut self, mut bytes: &[u8]) -> bool {
        let mut tries = 0;
        while !bytes.is_empty() && tries < FULL_TRIES {
            let chunk = &bytes[..bytes.len().min(MAX_BODY)];
            let Some(a) = self.call(WRITE, chunk).filter(|a| a.ok()) else { return false };
            let n = took_of(&a.body).unwrap_or(0) as usize;
            if n == 0 {
                tries += 1;
                let _ = mk_idle_ms(5);
            }
            bytes = &bytes[n.min(bytes.len())..];
        }
        bytes.is_empty()
    }
}

impl Drop for ServePipe {
    fn drop(&mut self) {
        let _ = self.call(CLOSE, &[]);
    }
}
