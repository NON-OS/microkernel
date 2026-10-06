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

//! Finding a function's AER capability in extended config space.

use super::decode::{ext_header, AER_CAP_ID};
use crate::drivers::pci::config::read_extended32;

/// The extended list cannot hold more entries than dwords above 100h. The
/// spec does not order the list, so this bound, as Linux's ttl in
/// pci_find_next_ext_capability, is what ends a looping chain.
const MAX_WALK: usize = (0x1000 - 0x100) / 4;

pub(super) fn aer_offset(bus: u8, device: u8, function: u8) -> Option<u16> {
    let mut at = 0x100u16;
    for _ in 0..MAX_WALK {
        let header = read_extended32(bus, device, function, at);
        if header == 0 || header == !0 {
            return None;
        }
        let (id, next) = ext_header(header);
        if id == AER_CAP_ID {
            return Some(at);
        }
        if next < 0x100 {
            return None;
        }
        at = next;
    }
    None
}
