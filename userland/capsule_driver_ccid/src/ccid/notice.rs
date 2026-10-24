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
//! RDR_to_PC_NotifySlotChange on the interrupt endpoint (CCID 1.1, 6.3.1,
//! as recalled, unverified): 50h, then two bits a slot from slot 0 up, the
//! low one a card present, the high one a change since the last.

const NOTIFY_SLOT_CHANGE: u8 = 0x50;

/// Whether notice `n` says slot `slot` changed since the last, if it
/// says anything of that slot.
pub fn changed(n: &[u8], slot: u8) -> Option<bool> {
    if n.first() != Some(&NOTIFY_SLOT_CHANGE) {
        return None;
    }
    let bit = 2 * slot as usize + 1;
    let byte = n.get(1 + bit / 8)?;
    Some(byte >> (bit % 8) & 1 != 0)
}
