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

//! Which of the shell's ops Settings may send on to the compositor, the
//! compositor op each becomes, and how long the answer's payload is (0 for
//! a status alone). Pure, so the proofs hold the table.

use crate::protocol::{
    OP_ACCESSIBILITY, OP_ACCESSIBILITY_GET, OP_DISPLAY_ARRANGEMENT, OP_DISPLAY_BRIGHTNESS,
    OP_DISPLAY_LAYOUT, OP_DISPLAY_SCALE,
};

/// NCMP's ACCESSIBILITY, ACCESSIBILITY_GET, OUTPUT_LAYOUT, OUTPUT_SCALE,
/// OUTPUT_ARRANGEMENT and BRIGHTNESS_GET, which only reads: Settings shows
/// each panel's level and never sets one.
pub const RELAY: [(u16, u16, usize); 6] = [
    (OP_ACCESSIBILITY, 0x000F, 0),
    (OP_ACCESSIBILITY_GET, 0x0010, 16),
    (OP_DISPLAY_LAYOUT, 0x000B, 0),
    (OP_DISPLAY_SCALE, 0x000D, 0),
    (OP_DISPLAY_ARRANGEMENT, 0x0011, 32),
    (OP_DISPLAY_BRIGHTNESS, 0x0016, 28),
];

/// The compositor op and answer length for shell op `op`, if Settings may
/// send it on.
pub fn relayed(op: u16) -> Option<(u16, usize)> {
    RELAY.iter().find(|r| r.0 == op).map(|r| (r.1, r.2))
}
