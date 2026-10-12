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

//! Where the linker put the kernel image's text and read-only data.

extern "C" {
    static __kernel_image_start: u8;
    static __kernel_text_start: u8;
    static __kernel_text_end: u8;
    static __kernel_rw_start: u8;
}

/// Text and read-only data, `[start, end)`: writable data starts the next
/// 2 MiB block.
pub(super) fn image_bounds() -> (u64, u64) {
    ((&raw const __kernel_image_start) as u64, (&raw const __kernel_rw_start) as u64)
}

pub(super) fn text_bounds() -> (u64, u64) {
    ((&raw const __kernel_text_start) as u64, (&raw const __kernel_text_end) as u64)
}
