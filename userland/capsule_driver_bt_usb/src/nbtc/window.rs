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

//! PAIR and SCAN, Settings' alone: when this host looks, and when a device
//! it holds no bond for may pair.

use super::board::Board;
use super::ops::{E_INVAL, E_OK, PAIR_MS};

/// PAIR, no payload: one new device may pair for `PAIR_MS`, and scanning
/// turns on to find it.
pub fn pair<K>(b: &mut Board<K>, now_ms: u64) -> i32 {
    (b.pair_until, b.scan) = (now_ms.saturating_add(PAIR_MS), true);
    E_OK
}

/// SCAN: 1 to scan, 0 to stop. Stopping shuts the pairing window too.
pub fn scan<K>(b: &mut Board<K>, body: &[u8]) -> i32 {
    match body {
        [on @ (0 | 1)] => {
            b.scan = *on == 1;
            if !b.scan {
                b.pair_until = 0;
            }
            E_OK
        }
        _ => E_INVAL,
    }
}
