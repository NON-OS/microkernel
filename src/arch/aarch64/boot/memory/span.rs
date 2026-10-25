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

//! Where the frame allocator's span starts, all addresses physical. Pure, so
//! the rule is held on the host (boot_memory_proofs).

/// The start of `[start, end)` for the allocator: past the kernel `image`
/// whenever the two meet, so no frame of the image is ever handed out. An
/// image reaching the region's end leaves nothing, which the caller reports.
pub(super) fn span_start((start, end): (u64, u64), (image_start, image_end): (u64, u64)) -> u64 {
    if image_start < end && image_end > start {
        image_end
    } else {
        start
    }
}
