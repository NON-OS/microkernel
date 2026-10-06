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

use crate::memory::layout::DIRECTMAP_BASE;

// Scrub the page through the kernel direct map before returning
// the frame to the global allocator. The next consumer of this
// frame must not see whatever the previous holder left there.
//
// SAFETY: eK@nonos.systems — `physical_start` came from
// `allocate_frame` and is only ever referenced through the broker
// grant table. The grant is removed from the records before this
// runs, so no other path can race on the same VA.
pub(super) fn scrub(physical_start: u64, length: u64) {
    let kva = (DIRECTMAP_BASE + physical_start) as *mut u64;
    let words = (length / 8) as usize;
    unsafe {
        for i in 0..words {
            core::ptr::write_volatile(kva.add(i), 0);
        }
    }
}
