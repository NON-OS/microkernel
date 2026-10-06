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

//! Which buttons went down or up between two reports. Pure, so the mapping
//! from report bits to button numbers is proven on the host.

/// Calls `emit(button, down)` for each button among the bits of `mask`
/// whose state differs between `previous` and `current`, lowest bit first.
/// Bit 0 is button 1. A bit outside `mask` never makes a button event.
pub fn button_changes(previous: u8, current: u8, mask: u8, mut emit: impl FnMut(u32, bool)) {
    let changed = (previous ^ current) & mask;
    for bit in 0..8u8 {
        let flag = 1u8 << bit;
        if changed & flag != 0 {
            emit(u32::from(bit) + 1, current & flag != 0);
        }
    }
}
