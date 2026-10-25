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

//! Where the frame allocator's span lies, all addresses physical. Pure, so
//! the rules are held on the host (riscv_kernel_proofs).

const PAGE: u64 = 0x1000;

/// `[base, base + size)` page aligned inward, or None when nothing is left
/// or the range wraps.
pub(crate) fn inward(base: u64, size: u64) -> Option<(u64, u64)> {
    let start = base.checked_add(PAGE - 1)? & !(PAGE - 1);
    let end = base.checked_add(size)? & !(PAGE - 1);
    (end > start).then_some((start, end))
}

/// The span's start in `[start, end)`: past the kernel `image`, page
/// aligned up, whenever the two meet, so no frame of the image is handed out.
/// A start at or past `end` leaves nothing, which the caller reports.
pub(crate) fn span_start((start, end): (u64, u64), (image_start, image_end): (u64, u64)) -> u64 {
    if image_start < end && image_end > start {
        image_end.saturating_add(PAGE - 1) & !(PAGE - 1)
    } else {
        start
    }
}
