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

/// Free bytes in a ring from `min` to `max` with the device at `stop` and
/// the driver at `next`, one word held back so a full ring never reads as
/// empty. None for an index outside the ring or not word aligned.
pub const fn free_bytes(next: u32, stop: u32, min: u32, max: u32) -> Option<u32> {
    let aligned = (next | stop | min | max) & 3 == 0;
    if !aligned || min >= max || next < min || next >= max || stop < min || stop >= max {
        return None;
    }
    let used = if next >= stop { next - stop } else { (max - min) - (stop - next) };
    Some(max - min - used - 4)
}
