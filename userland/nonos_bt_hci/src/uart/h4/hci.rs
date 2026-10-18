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

//! `Transport` over an H4 port. Commands and ACL data go out after their
//! packet indicator (Part A, 2); events and ACL data come in through one
//! reader and are kept apart until asked for. Synchronous and ISO data are
//! read to keep the packet edges, then dropped: this host is LE only and
//! opens neither.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::kind::{ACL, EVENT};
use super::reader::H4Reader;
use crate::error::HciError;
use crate::uart::port::UartPort;

/// Octets taken from the port in one look.
const READ_CHUNK: usize = 64;
/// Packets of one kind held for the host before the oldest is dropped.
pub const HELD_MAX: usize = 16;

pub struct H4Hci<P: UartPort> {
    pub(super) port: P,
    pub(super) reader: H4Reader,
    pub(super) events: VecDeque<Vec<u8>>,
    pub(super) acl: VecDeque<Vec<u8>>,
    pub(super) dropped: u32,
}

impl<P: UartPort> H4Hci<P> {
    pub fn new(port: P, acl_max: u16) -> Self {
        let (events, acl) = (VecDeque::new(), VecDeque::new());
        Self { port, reader: H4Reader::new(acl_max), events, acl, dropped: 0 }
    }

    // One look at the port, every whole packet in it sorted by kind.
    pub(super) fn pump(&mut self) -> Result<(), HciError> {
        let mut chunk = [0u8; READ_CHUNK];
        let n = self.port.read(&mut chunk).map_err(HciError::Transport)?;
        self.reader.push(&chunk[..n.min(READ_CHUNK)]);
        while let Some((kind, packet)) = self.reader.take()? {
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
        Ok(())
    }
}
