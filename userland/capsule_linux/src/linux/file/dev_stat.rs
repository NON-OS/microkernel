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

//! What stat, fstat and statx say about a character device: S_IFCHR with
//! read and write for everyone, as Linux's devtmpfs makes them, and the
//! device's major and minor numbers.

use super::dev::numbers;

const MODE: u32 = 0o020666;
/// st_mode and st_rdev in a `struct stat`.
const STAT_MODE: usize = 24;
const STAT_RDEV: usize = 40;
/// stx_mode, stx_rdev_major and stx_rdev_minor in a `struct statx`.
const STATX_MODE: usize = 28;
const STATX_RDEV: usize = 128;

/// A `struct stat` made for a file, turned into the device's. st_rdev is
/// Linux's encoding of the two numbers.
pub fn as_device(stat: &mut [u8], dev: u32) {
    let Some((major, minor)) = numbers(dev) else {
        return;
    };
    let rdev = (minor & 0xff) | ((major & 0xfff) << 8) | ((minor & !0xff) << 12);
    stat[STAT_MODE..STAT_MODE + 4].copy_from_slice(&MODE.to_le_bytes());
    stat[STAT_RDEV..STAT_RDEV + 8].copy_from_slice(&rdev.to_le_bytes());
}

/// The same for a `struct statx`, which keeps the two numbers apart.
pub fn statx_device(buf: &mut [u8], dev: u32) {
    let Some((major, minor)) = numbers(dev) else {
        return;
    };
    buf[STATX_MODE..STATX_MODE + 2].copy_from_slice(&(MODE as u16).to_le_bytes());
    buf[STATX_RDEV..STATX_RDEV + 4].copy_from_slice(&(major as u32).to_le_bytes());
    buf[STATX_RDEV + 4..STATX_RDEV + 8].copy_from_slice(&(minor as u32).to_le_bytes());
}
