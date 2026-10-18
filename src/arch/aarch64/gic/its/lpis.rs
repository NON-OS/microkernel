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

//! Handing out LPIs and event ids, a bit each.

use super::super::intid::LPI_BASE;
use super::state::LPI_WORDS;

pub(super) fn take_lpi(map: &mut [u64; LPI_WORDS]) -> Option<u32> {
    for (n, word) in map.iter_mut().enumerate() {
        if *word != u64::MAX {
            let bit = word.trailing_ones();
            *word |= 1 << bit;
            return Some(LPI_BASE + n as u32 * 64 + bit);
        }
    }
    None
}

pub(super) fn give_lpi(map: &mut [u64; LPI_WORDS], intid: u32) {
    let n = intid.wrapping_sub(LPI_BASE) as usize;
    if let Some(word) = map.get_mut(n / 64) {
        *word &= !(1 << (n % 64));
    }
}

/// The lowest free event of a device that has `1 << bits` of them.
pub(super) fn take_event(events: &mut u32, bits: u32) -> Option<u32> {
    let event = events.trailing_ones();
    if event >= (1 << bits).min(32) {
        return None;
    }
    *events |= 1 << event;
    Some(event)
}

pub(super) fn give_event(events: &mut u32, event: u32) {
    if event < 32 {
        *events &= !(1 << event);
    }
}
