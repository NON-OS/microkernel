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

//! A fill run the way the server runs it, against a device that answers
//! while the driver waits.

use super::ring::Ring;
use crate::constants::FILL_MS;
use crate::fill::fill;
use crate::queue::{Fault, Queue};
use crate::transport::Transport;

/// One fill the device answers, with `byte`, while the wait is under way.
pub fn answered(t: Transport, q: &mut Queue, ring: &Ring, byte: u8) -> Result<[u8; 64], Fault> {
    let (mut out, mut asked) = ([0u8; 64], 0u64);
    let tick = || {
        asked += 1;
        if asked == 2 {
            ring.answer(byte);
        }
        asked >= FILL_MS
    };
    assert_eq!(fill(t, q, &mut out, tick)?, 64);
    Ok(out)
}
