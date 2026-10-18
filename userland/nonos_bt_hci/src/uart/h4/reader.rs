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

//! Whole packets out of the octets an H4 port delivers, however the port
//! cuts them. A packet indicator that is not one a controller sends, or a
//! header stating more than its kind's bound, ends the stream: there is
//! no edge left to start again from (Part A, 3).

use alloc::vec::Vec;

use super::kind::{shape, stated};
use crate::error::HciError;
use nonos_bt_wire::WireError;

pub struct H4Reader {
    buf: Vec<u8>,
    acl_max: u16,
    lost: bool,
}

impl H4Reader {
    pub fn new(acl_max: u16) -> Self {
        Self { buf: Vec::new(), acl_max, lost: false }
    }

    pub fn set_acl_max(&mut self, acl_max: u16) {
        self.acl_max = acl_max;
    }

    pub fn push(&mut self, octets: &[u8]) {
        self.buf.extend_from_slice(octets);
    }

    /// The next whole packet's indicator and the packet after it, `None`
    /// until one is whole. Once the stream is lost every call says so.
    pub fn take(&mut self) -> Result<Option<(u8, Vec<u8>)>, HciError> {
        if self.lost {
            return self.lose(WireError::Reserved);
        }
        let Some(&indicator) = self.buf.first() else { return Ok(None) };
        let Some((header, max)) = shape(indicator, self.acl_max) else {
            return self.lose(WireError::Reserved);
        };
        let Some(head) = self.buf.get(1..1 + header) else { return Ok(None) };
        let total = header + stated(indicator, head);
        if total > max {
            return self.lose(WireError::TooLong);
        }
        if self.buf.len() < 1 + total {
            return Ok(None);
        }
        let rest = self.buf.split_off(1 + total);
        let mut packet = core::mem::replace(&mut self.buf, rest);
        packet.remove(0);
        Ok(Some((indicator, packet)))
    }

    fn lose<T>(&mut self, why: WireError) -> Result<T, HciError> {
        (self.lost, self.buf) = (true, Vec::new());
        Err(HciError::Wire(why))
    }
}
