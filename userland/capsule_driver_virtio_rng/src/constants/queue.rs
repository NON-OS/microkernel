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

//! Virtqueue layout and buffer sizing. One grant of two pages holds the
//! ring: descriptors from the start, the available ring right after a table
//! of `queue_size` descriptors, the used ring on the second page, which is
//! where the legacy layout puts it for every size up to `LEGACY_MAX`.

/// The ring the modern transport asks for. A legacy device dictates its
/// own size, which is taken as offered when the region holds it.
pub const QUEUE_SIZE: u16 = 16;
pub const VRING_DESC_F_WRITE: u16 = 2;

pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_USED_OFFSET: usize = 4096;
pub const VQ_REGION_SIZE: usize = 8192;

/// The largest legacy ring whose descriptors and available ring end before
/// the used ring's page: 16 bytes a descriptor, 2 an available entry, and 6
/// of index, flags and used_event.
pub const LEGACY_MAX: u16 = ((VQ_USED_OFFSET - 6) / 18) as u16;

pub const ENTROPY_BUF_LEN: usize = 4096;

// The used ring of a LEGACY_MAX ring still ends inside the region.
const _: () = assert!(VQ_USED_OFFSET + 6 + 8 * LEGACY_MAX as usize <= VQ_REGION_SIZE);
