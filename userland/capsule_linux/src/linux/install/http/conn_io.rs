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

//! A connection's sends, and its reads without blocking, so one loop can
//! serve several (get/window.rs).

use alloc::vec::Vec;

use super::conn::Conn;
#[cfg(feature = "dev-mirror")]
use crate::linux::net::{raw, raw_io};

/// How long one read waits on a route stream before the loop looks at the next.
const POLL_MS: u64 = 20;
/// The most one read takes.
const CHUNK: usize = 32 << 10;

impl Conn {
    pub fn send(&mut self, req: &[u8]) -> Result<(), &'static str> {
        match self {
            Conn::Route(s) => s.0.write_all(req),
            #[cfg(feature = "dev-mirror")]
            Conn::Raw(h) => raw_io::send_all(*h, req).ok_or("the send failed"),
        }
    }

    /// What has come since the last read, onto `into`: zero when nothing yet.
    pub fn poll(&mut self, into: &mut Vec<u8>) -> Result<usize, &'static str> {
        let mut chunk = alloc::vec![0u8; CHUNK];
        let n = match self {
            Conn::Route(s) => s.0.read_wait(&mut chunk, POLL_MS)?,
            #[cfg(feature = "dev-mirror")]
            Conn::Raw(h) => {
                // A refused read is "nothing yet", as raw_io::recv_until takes
                // it: a mirror fetching upstream is quiet, not gone.
                let got = raw::recv_now(*h, CHUNK).unwrap_or_default();
                chunk[..got.len()].copy_from_slice(&got);
                got.len()
            }
        };
        into.extend_from_slice(&chunk[..n]);
        Ok(n)
    }
}
