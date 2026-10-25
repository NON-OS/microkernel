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

//! How many bytes one surface may span. The kernel bounds a surface to
//! bound the work behind it: one page-table walk and one frame entry per page.
//! 64 MiB holds a 4K panel (about 33 MiB). A larger boot framebuffer, a 5K, 6K
//! or 8K panel, raises the cap to its own size rounded up to a page, so the
//! compositor's full-size backing registers; never past 128 MiB, which holds
//! 8K (7680x4320, 132,710,400 bytes). Headless, the cap stays 64 MiB. Pure:
//! kernel_proofs::surface_cap.

/// The cap with no boot framebuffer, or one that fits under it.
pub(crate) const FLOOR: u64 = 64 * 1024 * 1024;
/// No surface spans more, whatever the framebuffer: 32,768 pages.
pub(crate) const CEILING: u64 = 128 * 1024 * 1024;
const PAGE: u64 = 4096;

/// The cap for a boot framebuffer of `frame_len` bytes (stride x height), or
/// none when the machine is headless.
pub(crate) const fn surface_cap(frame_len: Option<u64>) -> u64 {
    let Some(len) = frame_len else {
        return FLOOR;
    };
    let rounded = match len.checked_add(PAGE - 1) {
        Some(n) => n & !(PAGE - 1),
        None => return CEILING,
    };
    if rounded <= FLOOR {
        FLOOR
    } else if rounded >= CEILING {
        CEILING
    } else {
        rounded
    }
}
