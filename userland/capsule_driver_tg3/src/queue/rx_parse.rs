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

//! What one return-ring entry says, decided from its three words alone.
//! Every word is the part's to write, so nothing here trusts it: the ring
//! must be the standard one and the index inside it, and the length must
//! hold an Ethernet frame once the FCS is taken off (tg3_rx).

use crate::constants::desc::*;
use crate::constants::ring::RX_STD_RING_SIZE;
use crate::constants::{FCS_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Entry {
    /// Not a standard-ring slot this driver could have posted.
    Foreign,
    /// Slot `idx` came back with an error or a length out of bounds.
    Drop(u16),
    /// Slot `idx` holds a whole frame of `len` bytes.
    Frame(u16, u16),
}

pub fn parse(opaque: u32, idx_len: u32, err_vlan: u32) -> Entry {
    let idx = opaque & RXD_OPAQUE_INDEX_MASK;
    if opaque & RXD_OPAQUE_RING_MASK != RXD_OPAQUE_RING_STD || idx as usize >= RX_STD_RING_SIZE {
        return Entry::Foreign;
    }
    let idx = idx as u16;
    if err_vlan & RXD_ERR_MASK != 0 {
        return Entry::Drop(idx);
    }
    let len = (idx_len & RXD_LEN_MASK) as usize;
    match len.checked_sub(FCS_LEN) {
        Some(n) if (MIN_ETHERNET_FRAME..=MAX_ETHERNET_FRAME).contains(&n) => {
            Entry::Frame(idx, n as u16)
        }
        _ => Entry::Drop(idx),
    }
}
