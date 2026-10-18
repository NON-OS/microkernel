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

use super::PAGE_SIZE;

extern "C" {
    static __kernel_image_start: u8;
    static __kernel_image_end: u8;
}

/// The kernel image as whole pages, `[start, end)`: text, data, BSS and the
/// boot page tables in it, none of which any allocator may hand out.
pub(super) fn bounds() -> (u64, u64) {
    let start = (&raw const __kernel_image_start) as u64 & !(PAGE_SIZE - 1);
    let end = (&raw const __kernel_image_end) as u64;
    (start, (end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1))
}
