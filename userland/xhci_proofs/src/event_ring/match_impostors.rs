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

//! Which event completes which request.
//!
//! A command completes on the Command Completion Event whose pointer names
//! that command's TRB; a transfer completes on the Transfer Event whose
//! pointer names the TRB that asked for the interrupt and whose slot and
//! endpoint ids name the ring that TRB is on (xHCI 1.2 section 6.4.2.1).
//! Anything else that arrives first, whatever its completion code says, must
//! not finish the wait. Each test puts the impostors ahead of the real event
//! with a failing completion code or a different residual, so taking any of
//! them shows as an error or a wrong length rather than passing quietly.

use super::fixture::{EP0_DCI, INT_DCI, SLOT, TRB};

/// Pointers that are not `issued`: its neighbours on the ring, the same
/// address with the high half changed, the null pointer, and a page away.
pub fn not_issued(issued: u64) -> [u64; 5] {
    [issued + TRB, issued - TRB, issued ^ (1 << 40), 0, issued + 0x1000]
}

/// Slot and endpoint ids other than (`SLOT`, `dci`): slot 0, which is never
/// a device; slot 255, past any MaxSlots; a slot never enabled; endpoint 0,
/// which no DCI is; endpoint 31, the last; and another endpoint of the slot.
pub fn not_configured(dci: u8) -> [(u8, u8); 6] {
    let other = if dci == INT_DCI { EP0_DCI } else { INT_DCI };
    [(0, dci), (255, dci), (SLOT + 1, dci), (SLOT, 0), (SLOT, 31), (SLOT, other)]
}
