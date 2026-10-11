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

//! Reading the return ring up to the producer index the part last wrote in
//! the status block. A producer past the ring, an entry naming another ring
//! or a slot that carries no buffer is the part's error, never a reason to
//! touch memory outside the rings: such an entry is skipped.

use core::ptr::{addr_of, read_volatile};

use crate::constants::ring::RX_RET_RING_SIZE;

use super::rx::RxQueue;
use super::rx_parse::{parse, Entry};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rx {
    /// A whole frame of `len` bytes in buffer `buf`.
    Frame { buf: u8, len: u16 },
    /// Buffer `buf` came back with nothing to hand up.
    Dropped { buf: u8 },
    /// An entry with no buffer of this driver's behind it.
    Skipped,
}

impl RxQueue {
    pub fn consume(&mut self, hw_prod: u16) -> Option<Rx> {
        if hw_prod as usize >= RX_RET_RING_SIZE || self.ret_cons == hw_prod {
            return None;
        }
        let bd = self.ret_bd(self.ret_cons as usize);
        // SAFETY: `ret_bd` reduced the index into the return ring (rx.rs).
        let (opaque, idx_len, err_vlan) = unsafe {
            (
                read_volatile(addr_of!((*bd).opaque)),
                read_volatile(addr_of!((*bd).idx_len)),
                read_volatile(addr_of!((*bd).err_vlan)),
            )
        };
        self.ret_cons = ((self.ret_cons as usize + 1) % RX_RET_RING_SIZE) as u16;
        let (idx, len) = match parse(opaque, idx_len, err_vlan) {
            Entry::Foreign => return Some(Rx::Skipped),
            Entry::Drop(idx) => (idx, None),
            Entry::Frame(idx, len) => (idx, Some(len)),
        };
        let Some(buf) = self.carries[idx as usize].take() else {
            return Some(Rx::Skipped);
        };
        Some(match len {
            Some(len) => Rx::Frame { buf, len },
            None => Rx::Dropped { buf },
        })
    }
}
