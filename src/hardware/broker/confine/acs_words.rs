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

//! The ACS words of a function, read and turned on by layout, pure over
//! its register reads and writes (K11b). kernel_proofs::acs_group.

use super::acs_layout::Layout;
use super::acs_rule::enabling;

/// The capability and control words of a function laid out as `layout`,
/// with `needed` turned on first where offered. `read` and `write` take
/// offsets past the ACS header; `has_cap`: whether it has one. An RCiEP is
/// read as offering and enabling all of `needed`; anything else without
/// the capability has no words.
pub(crate) fn words(
    layout: Layout,
    needed: u16,
    has_cap: bool,
    read: impl Fn(u16) -> u32,
    mut write: impl FnMut(u16, u32),
) -> Option<(u16, u16)> {
    match layout {
        Layout::Rciep => Some((needed, needed)),
        _ if !has_cap => None,
        Layout::Standard => {
            let (cap, ctrl) = (read(4) as u16, (read(4) >> 16) as u16);
            let on = enabling(cap, ctrl, needed);
            if on != ctrl {
                write(4, u32::from(cap) | u32::from(on) << 16);
            }
            Some((read(4) as u16, (read(4) >> 16) as u16))
        }
        Layout::SptDword => {
            let (cap, ctrl) = (read(4) as u16, read(8));
            let on = enabling(cap, ctrl as u16, needed);
            if on != ctrl as u16 {
                write(8, (ctrl & !0xFFFF) | u32::from(on));
            }
            Some((read(4) as u16, read(8) as u16))
        }
    }
}
