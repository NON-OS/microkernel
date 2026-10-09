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

use super::clock::{line, Moment, LINE_MAX};
use super::local_time;

pub const STAMP_LEN: usize = LINE_MAX;

/// The menubar's date and time now, in `buf`, and its length; None while
/// the clock cannot be read.
pub fn stamp(buf: &mut [u8; STAMP_LEN], h24: bool, offset_hours: i8) -> Option<usize> {
    let t = local_time::now(offset_hours)?;
    let m = Moment { year: t.year, month: t.month, day: t.day, hour: t.hour, minute: t.minute };
    Some(line(buf, &m, h24))
}
