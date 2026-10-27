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
 * A buffer a Wayland client makes with shm_open: a new name straight under
 * /dev/shm, created exclusively and opened to read and write, which the
 * client unlinks at once and maps shared to draw a window into. That is a
 * memfd in all but its name, and a memfd is what the compositor bridge can
 * share; a shared mapping of a stored file is not served.
 */

const O_ACCMODE: u64 = 0o3;
const O_RDWR: u64 = 0o2;
const O_CREAT: u64 = 0o100;
const O_EXCL: u64 = 0o200;
const SHM: &[u8] = b"/dev/shm/";

pub fn is_buffer(full: &[u8], flags: u64) -> bool {
    let fresh = flags & (O_CREAT | O_EXCL) == O_CREAT | O_EXCL;
    let name = match full.strip_prefix(SHM) {
        Some(name) => name,
        None => return false,
    };
    fresh && flags & O_ACCMODE == O_RDWR && !name.is_empty() && !name.contains(&b'/')
}
