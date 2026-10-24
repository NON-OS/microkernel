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

use super::call::call;
use super::message::{answer, Tag};

/// Get clock rate: one value in, the clock's id; two out, id and Hz.
const GET_CLOCK_RATE: u32 = 0x0003_0002;
/// The firmware's id for the eMMC2 controller's clock.
pub const CLOCK_EMMC2: u32 = 0xc;

/// The rate the firmware runs clock `id` at, in Hz. `None` without a
/// mailbox, or when it does not answer or names the clock off.
pub fn clock_rate(id: u32) -> Option<u32> {
    let tags = [Tag { id: GET_CLOCK_RATE, values: &[id], room: 2 }];
    let words = call(&tags)?;
    match answer(&tags, &words, 0).ok()? {
        [answered, hz, ..] if *answered == id && *hz != 0 => Some(*hz),
        _ => None,
    }
}
