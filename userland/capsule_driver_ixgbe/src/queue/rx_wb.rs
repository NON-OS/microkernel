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

//! What a written-back receive descriptor says, every bit the part's word
//! (positions in facts::rx, as recalled, unverified). Without DD it is not
//! yet written. Without EOP it is a piece of a frame longer than one 2 KiB
//! buffer, which this driver never joins. Any frame error bit, a length of
//! 0, or one past an Ethernet frame drops it; the frame bound is under the
//! posted buffer's, so the copy out of the slot never leaves it.

use crate::facts::rx::{RXDEXT_ERR_FRAME_ERR_MASK, RXD_STAT_DD, RXD_STAT_EOP};
use crate::sizes::ring::RX_BUFFER_LEN;
use crate::sizes::MAX_ETHERNET_FRAME;

const _: () = assert!(MAX_ETHERNET_FRAME <= RX_BUFFER_LEN);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rx {
    /// The part has not written this slot back yet.
    NotDone,
    /// Written back, but nothing to hand up: the slot is recycled.
    Drop,
    /// Written back without EOP: one piece of a longer frame.
    Piece,
    /// A whole, error-free frame of this many bytes.
    Frame(u16),
}

pub fn parse(staterr: u32, length: u16) -> Rx {
    if staterr & RXD_STAT_DD == 0 {
        return Rx::NotDone;
    }
    if staterr & RXD_STAT_EOP == 0 {
        return Rx::Piece;
    }
    if staterr & RXDEXT_ERR_FRAME_ERR_MASK != 0
        || length == 0
        || length as usize > MAX_ETHERNET_FRAME
    {
        return Rx::Drop;
    }
    Rx::Frame(length)
}
