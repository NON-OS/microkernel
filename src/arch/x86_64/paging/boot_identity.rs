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

//! Whether the loader's low-half identity mapping is still installed. Early
//! diagnostics write the firmware framebuffer through it, and must stop the
//! moment clear_low_half takes it away.

use core::sync::atomic::{AtomicBool, Ordering};

static LIVE: AtomicBool = AtomicBool::new(true);

/// True from kernel entry until the low half is first cleared.
pub fn boot_identity_live() -> bool {
    LIVE.load(Ordering::Acquire)
}

/// Called once the low-half PML4 slots are zeroed and CR3 reloaded.
pub(super) fn mark_boot_identity_gone() {
    LIVE.store(false, Ordering::Release);
}
