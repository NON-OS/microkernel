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

use super::{total_free_frames, PAGE_SIZE_U64};

/*
 * Bytes of RAM free to allocate. This used to return the managed span less
 * the free bytes, which is what is in use plus every hole in the span, and
 * /proc/meminfo printed that as MemFree.
 */
pub fn free_memory() -> u64 {
    (total_free_frames() as u64).saturating_mul(PAGE_SIZE_U64)
}
