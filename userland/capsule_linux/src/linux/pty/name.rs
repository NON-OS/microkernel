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

//! How a pseudo-terminal descriptor names its pair, and the names a guest
//! opens one by: /dev/ptmx for a new master, /dev/pts/N for a slave. Pure.

use alloc::vec::Vec;

/// A descriptor's handle: the pair's number, and whether it is the master.
pub fn handle(n: u32, master: bool) -> u32 {
    n << 1 | u32::from(master)
}

pub fn of_handle(h: u32) -> (u32, bool) {
    (h >> 1, h & 1 != 0)
}

/// The number /dev/pts/N names: decimal, with no sign and no leading zero.
pub fn pts_number(path: &[u8]) -> Option<u32> {
    let digits = path.strip_prefix(b"/dev/pts/")?;
    let lead = digits.first()?;
    if !digits.iter().all(u8::is_ascii_digit) || (*lead == b'0' && digits.len() > 1) {
        return None;
    }
    core::str::from_utf8(digits).ok()?.parse().ok()
}

/// What ptsname gives for pair `n`, and the path its descriptor carries.
pub fn pts_path(n: u32) -> Vec<u8> {
    alloc::format!("/dev/pts/{n}").into_bytes()
}
