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

use core::arch::asm;

/// Drop every instruction cache line to the point of unification, so this
/// CPU fetches nothing an earlier stage left behind.
pub(super) fn configure_cache() {
    // SAFETY: cache maintenance and barriers; no memory is written.
    unsafe {
        asm!("ic iallu", options(nostack));
        asm!("dsb ish", options(nostack));
        asm!("isb", options(nostack));
    }
}
