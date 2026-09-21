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

//! Reading and writing cells over an established link.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::cell::{parse, Frame};

use super::session::{Link, LinkError};

impl Link {
    /// Send one encoded cell.
    pub fn send(&mut self, bytes: &[u8]) -> Result<(), LinkError> {
        self.stream.write_all(&mut self.socket, bytes).map_err(|_| LinkError::Tls)
    }

    /// Next cell, or `None` if none has arrived inside `wait_ms`.
    ///
    pub fn recv(&mut self, wait_ms: i64) -> Result<Option<Frame>, LinkError> {
        if let Some(frame) = self.held.pop_front() {
            return Ok(Some(frame));
        }
        let deadline = mk_uptime_ms().saturating_add(wait_ms);
        loop {
            if let Some((frame, used)) = parse(&self.partial) {
                self.partial.drain(..used);
                return Ok(Some(frame));
            }
            if self.stream.is_done() {
                return Err(LinkError::Closed);
            }
            let more: Vec<u8> = self.stream.read(&mut self.socket).map_err(|_| LinkError::Tls)?;
            if more.is_empty() {
                if mk_uptime_ms() >= deadline {
                    return Ok(None);
                }
                mk_yield();
                continue;
            }
            self.partial.extend_from_slice(&more);
        }
    }
}
