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

//! The setup frame that loads the part's perfect filter: sixteen entries of
//! twelve bytes, each address as three sixteen-bit halves in the low half
//! of a longword. Entry 0 is the station address, entry 1 broadcast, and
//! the rest repeat the station address, so no other address is received.

use crate::constants::queue::SETUP_FRAME_LEN;
use crate::constants::MAC_LEN;

const ENTRIES: usize = 16;
const BROADCAST: [u8; MAC_LEN] = [0xFF; MAC_LEN];

pub fn encode(mac: &[u8; MAC_LEN]) -> [u8; SETUP_FRAME_LEN] {
    let mut f = [0u8; SETUP_FRAME_LEN];
    for e in 0..ENTRIES {
        let addr = if e == 1 { &BROADCAST } else { mac };
        for half in 0..3 {
            let at = e * 12 + half * 4;
            f[at] = addr[2 * half];
            f[at + 1] = addr[2 * half + 1];
        }
    }
    f
}
