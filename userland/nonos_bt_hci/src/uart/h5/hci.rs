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

//! `Transport` over a Three-wire UART port. Each look reads what has come,
//! sends what is due, and sorts the HCI packets taken. The link is lost
//! when the peer stops acknowledging, or starts over under a host using
//! it: the controller then knows nothing of what the host has done.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::engine::H5;
use super::header::{ACL, EVENT};
use crate::error::HciError;
use crate::uart::h4::HELD_MAX;
use crate::uart::port::UartPort;

const READ_CHUNK: usize = 64;
/// Packets written in one look, so a look always ends.
const WRITES_MAX: usize = 8;

pub struct H5Hci<P: UartPort> {
    pub(super) port: P,
    pub(super) h5: H5,
    pub(super) events: VecDeque<Vec<u8>>,
    pub(super) acl: VecDeque<Vec<u8>>,
    pub(super) retx_ms: u64,
    pub(super) restarts: u32,
    pub dropped: u32,
}

impl<P: UartPort> H5Hci<P> {
    pub(super) fn pump(&mut self) -> Result<(), HciError> {
        let mut chunk = [0u8; READ_CHUNK];
        let n = self.port.read(&mut chunk).map_err(HciError::Transport)?;
        let now = self.port.now_ms();
        self.h5.feed(&chunk[..n.min(READ_CHUNK)], now);
        for _ in 0..WRITES_MAX {
            let Some(o) = self.h5.poll_out(now, self.retx_ms) else { break };
            self.port.write(&o).map_err(HciError::Transport)?;
        }
        while let Some((kind, packet)) = self.h5.inbox.pop_front() {
            // No command comes to a host, and it opens no synchronous link.
            let queue = match kind {
                EVENT => &mut self.events,
                ACL => &mut self.acl,
                _ => continue,
            };
            if queue.len() == HELD_MAX {
                queue.pop_front();
                self.dropped = self.dropped.saturating_add(1);
            }
            queue.push_back(packet);
        }
        if self.h5.rel.lost(now) || self.h5.restarts != self.restarts {
            return Err(HciError::NoLink);
        }
        Ok(())
    }
}
