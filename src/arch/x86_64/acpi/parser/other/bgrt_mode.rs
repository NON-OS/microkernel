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

//! The display mode firmware's GOP was in when BGRT was read: the logo's
//! x and y are offsets into that mode's screen (ACPI 6.5, 5.2.23), so a
//! reader in another mode must know it to place the logo where it was.
//! Taken from the boot handoff's framebuffer; 0 by 0 when there was none.

use core::sync::atomic::{AtomicU64, Ordering};

static MODE: AtomicU64 = AtomicU64::new(0);

/// Called once the logo is kept.
pub(super) fn record() {
    let fb = crate::boot::handoff::get_handoff().and_then(|h| h.framebuffer().copied());
    let (w, h) = fb.map_or((0, 0), |f| (f.width, f.height));
    MODE.store(u64::from(w) << 32 | u64::from(h), Ordering::Release);
}

/// The mode's width and height in pixels.
pub fn firmware_mode() -> (u32, u32) {
    let m = MODE.load(Ordering::Acquire);
    ((m >> 32) as u32, m as u32)
}
