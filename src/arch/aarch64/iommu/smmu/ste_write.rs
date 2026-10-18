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

//! Writing an STE the unit may be reading. Word 0 holds the valid bit and
//! the configuration, so its order relative to the rest decides what the
//! unit can see half-way.

use super::encode::ste::Ste;
use super::wait;

const CONFIG_MASK: u64 = 0b1110;

/// Taking translation away, word 0 goes first: the stream aborts at once
/// and the stale pointers behind it no longer matter. Granting it, word 0
/// goes last, so the unit never sees a translating STE with an old CD.
pub(super) fn write(slot: &mut [u64], ste: &Ste) {
    if ste[0] & CONFIG_MASK == 0 {
        slot[0] = ste[0];
        wait::publish();
        slot[1..].copy_from_slice(&ste[1..]);
    } else {
        slot[1..].copy_from_slice(&ste[1..]);
        wait::publish();
        slot[0] = ste[0];
    }
    wait::publish();
}
