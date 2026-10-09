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

use super::display_free;
use super::low32_free::{low32_free, low32_owns};

/// Return a run to the pool that handed it out. `false` when no pool owns it,
/// so the caller frees it to the general allocator; a pool run must never go
/// there, since its frames stay reserved in the allocator's bitmap. A pool
/// run the pool refuses stays marked handed out: a leak, said on the log,
/// rather than one frame handed to two devices.
pub(in crate::hardware::broker::dma) fn give_back(addr: u64, pages: usize) -> bool {
    let freed = if low32_owns(addr) {
        low32_free(addr, pages)
    } else {
        match display_free::free(addr, pages) {
            Some(freed) => freed,
            None => return false,
        }
    };
    if !freed {
        crate::sys::serial::print(b"[DMA] pool refused a free of a run it did not hand out: addr=");
        crate::sys::serial::print_hex(addr);
        crate::sys::serial::print(b" pages=");
        crate::sys::serial::print_dec(pages as u64);
        crate::sys::serial::println(b"; frames kept");
    }
    true
}
