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

//! LINKS decoded: link up, and the speed field, which on the X550 with the
//! non-standard bit set means the NBASE-T rates (2.5 Gb/s in the 10 Gb/s
//! code, 5 Gb/s in the 100 Mb/s one; as observed in Linux ixgbe,
//! unverified). Every ixgbe link is full duplex.

use crate::facts::mac::{LINKS_SPEED_MASK, LINKS_SPEED_NON_STD, LINKS_SPEED_SHIFT, LINKS_UP};
use crate::facts::parts::Part;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkState {
    pub up: bool,
    pub mbps: u16,
}

pub fn decode(links: u32, part: Part) -> LinkState {
    let nbase_t = part.has_nbase_t() && links & LINKS_SPEED_NON_STD != 0;
    let mbps = match ((links >> LINKS_SPEED_SHIFT) & LINKS_SPEED_MASK, nbase_t) {
        (3, false) => 10_000,
        (3, true) => 2_500,
        (2, _) => 1_000,
        (1, false) => 100,
        (1, true) => 5_000,
        _ => 10,
    };
    LinkState { up: links & LINKS_UP != 0, mbps }
}

/// Whether `now` is worth a line after `before`. The speed field means
/// nothing while the link is down, so two downs are the same state.
pub fn changed(before: Option<LinkState>, now: LinkState) -> bool {
    match before {
        None => true,
        Some(b) if !b.up && !now.up => false,
        Some(b) => b != now,
    }
}
