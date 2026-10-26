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

//! The refusal decided with the posture, carried out once the kernel can draw
//! it. The posture is decided right after paging, when the loader's mapping of
//! the panel is gone and the kernel's is not made yet, so a stop there said
//! why only on serial and left the loader's "Starting" on the panel. Held
//! here, it is carried out as the framebuffer is mapped, still before any
//! process, a driver among them, can exist.

use core::sync::atomic::{AtomicU8, Ordering};

use super::no_iommu_refusal::{Needs, ALL};

/// 0: no refusal; n: ALL[n - 1].
static HELD: AtomicU8 = AtomicU8::new(0);

pub(super) fn hold(needs: Needs) {
    let at = ALL.iter().position(|&n| n == needs).unwrap_or(0);
    HELD.store(at as u8 + 1, Ordering::Release);
}

/// Stop the boot if the posture decision refused it.
pub fn stop_if_refused() {
    let held = HELD.load(Ordering::Acquire) as usize;
    if let Some(needs) = held.checked_sub(1).and_then(|at| ALL.get(at)) {
        crate::boot::stop("IOMMU", needs.line());
    }
}
