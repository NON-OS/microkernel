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

//! The command at a slot of the available ring, as the part reads it.

use nonos_devmodel::FakeBar;

use super::ring::{bytes, desc, region_at, u16_at};
use crate::constants::{VQ_AVAIL_OFFSET, VQ_DESC_OFFSET};
use crate::device::cmd::hdr::HDR_LEN;

pub struct Chain {
    pub head: u16,
    pub request: Vec<u8>,
    /// The flags on the request's descriptor and on the answer's.
    pub flags: (u16, u16),
    /// Where the answer goes in the region, and how many bytes it may take.
    pub answer_at: usize,
    pub room: usize,
}

/// The chain the head at `slot` names, or None when it does not lie in the
/// region or its request is shorter than a header (`ring::region_at`).
pub fn read(bar: &FakeBar, slot: usize) -> Option<Chain> {
    let head = u16_at(bar, VQ_AVAIL_OFFSET + 4 + slot * 2);
    let first = desc(bar, head, VQ_DESC_OFFSET);
    let second = desc(bar, first.next, VQ_DESC_OFFSET);
    let (len, room) = (first.len as usize, second.len as usize);
    let (from, answer_at) = (region_at(bar, first.addr, len)?, region_at(bar, second.addr, room)?);
    let request = bytes(bar, from, len);
    let flags = (first.flags, second.flags);
    (len >= HDR_LEN).then_some(Chain { head, request, flags, answer_at, room })
}
