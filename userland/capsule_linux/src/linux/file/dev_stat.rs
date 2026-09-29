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

/*
 * What stat, fstat and statx say about a character device: S_IFCHR with
 * read and write for everyone, as Linux's devtmpfs makes them, and the
 * device's major and minor numbers.
 */

use super::dev::numbers;

pub const MODE: u32 = 0o020666;

/* Linux's st_rdev for the device: the minor's low byte, the major, the rest. */
pub fn rdev(dev: u32) -> Option<u64> {
    let (major, minor) = numbers(dev)?;
    Some((minor & 0xff) | ((major & 0xfff) << 8) | ((minor & !0xff) << 12))
}
