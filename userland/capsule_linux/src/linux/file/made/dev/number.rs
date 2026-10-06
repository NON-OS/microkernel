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

/* The device numbers stat reports, and /dev/fd's links. */

use super::tree::{Dev, DEVICES};

/* st_rdev, in the encoding glibc's and musl's makedev use. */
pub fn rdev(dev: Dev) -> u64 {
    let (_, _, major, minor) = DEVICES.iter().find(|d| d.1 == dev).copied().unwrap_or(DEVICES[0]);
    let (major, minor) = (u64::from(major), u64::from(minor));
    ((major & 0xfff) << 8) | (minor & 0xff) | ((minor & !0xff) << 12) | ((major & !0xfff) << 32)
}

/* The device a path names, for a descriptor already open on it. */
pub fn at(path: &[u8]) -> Option<Dev> {
    let name = path.strip_prefix(b"/dev/")?;
    DEVICES.iter().find(|d| d.0 == name).map(|d| d.1)
}
