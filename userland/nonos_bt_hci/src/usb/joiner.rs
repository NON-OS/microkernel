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

//! Whole HCI packets out of USB transfers. An event longer than the
//! interrupt endpoint's packet size comes in several transfers, and ACL
//! data the same way on bulk (Vol 4, Part B, 2.1), so the bytes are joined
//! and cut by the length the packet's own header states: one octet at
//! offset 1 for an event (Vol 4, Part E, 5.4.4), two at offset 2 for ACL
//! data (Vol 4, Part E, 5.4.2).

use alloc::vec::Vec;

use crate::error::HciError;
use nonos_bt_wire::WireError;

pub struct Joiner {
    buf: Vec<u8>,
    header: usize,
    max: usize,
}

impl Joiner {
    pub fn events() -> Self {
        Self { buf: Vec::new(), header: 2, max: 2 + 255 }
    }

    /// ACL packets of at most `data_max` data octets.
    pub fn acl(data_max: u16) -> Self {
        Self { buf: Vec::new(), header: 4, max: 4 + usize::from(data_max) }
    }

    /// Add one transfer's bytes.
    pub fn push(&mut self, chunk: &[u8]) {
        self.buf.extend_from_slice(chunk);
    }

    /// The next whole packet, `None` until one is. A header that states a
    /// packet over the bound leaves no packet edge to start again from, so
    /// everything held is dropped with it.
    pub fn take(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        let Some(head) = self.buf.get(..self.header) else { return Ok(None) };
        let len = match self.header {
            2 => usize::from(head[1]),
            _ => usize::from(u16::from_le_bytes([head[2], head[3]])),
        };
        let total = self.header + len;
        if total > self.max {
            self.buf.clear();
            return Err(HciError::Wire(WireError::TooLong));
        }
        if self.buf.len() < total {
            return Ok(None);
        }
        let rest = self.buf.split_off(total);
        Ok(Some(core::mem::replace(&mut self.buf, rest)))
    }

    /// Octets held toward a packet not yet whole.
    pub fn held(&self) -> usize {
        self.buf.len()
    }
}
