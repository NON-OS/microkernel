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

//! Reading device configuration wider than one access.
//!
//! A field read in pieces (a MAC address, a 64-bit capacity) can change
//! between the pieces. The device bumps config_generation when it does, so
//! the read is repeated until the generation is the same on both sides.

use super::access::CommonCfg;
use super::regs::CONFIG_GENERATION;

pub const GENERATION_TRIES: u32 = 8;

/// `read`'s value from a pass the generation did not move across, or
/// `None` after `GENERATION_TRIES` passes that it did.
pub fn stable_read<T>(c: &impl CommonCfg, mut read: impl FnMut() -> T) -> Option<T> {
    for _ in 0..GENERATION_TRIES {
        let before = c.r8(CONFIG_GENERATION);
        let value = read();
        if c.r8(CONFIG_GENERATION) == before {
            return Some(value);
        }
    }
    None
}
