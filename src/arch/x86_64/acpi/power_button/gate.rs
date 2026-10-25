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

//! Whether the tick may read the button, and whether a press is a new one.

use core::sync::atomic::Ordering;

use super::super::hw::power_button::fresh;
use super::LAST_PRESS_MS;

/// The ACPI capsule's SCI reads the button while that capsule lives, and
/// runs the wiped power-off on a press; both reading would lose presses.
pub(super) fn capsule_serves() -> bool {
    #[cfg(feature = "nonos-capsule-acpi")]
    let alive = crate::userspace::capsule_acpi::shared_state().pid();
    // Without the capsule in the image nothing can take the button.
    #[cfg(not(feature = "nonos-capsule-acpi"))]
    let alive = 0;
    crate::hardware::broker::acpi::button_owner::stands_down(alive)
}

pub(super) fn accept(now_ms: u64) -> bool {
    let last = LAST_PRESS_MS.load(Ordering::Relaxed);
    if !fresh((last != u64::MAX).then_some(last), now_ms) {
        return false;
    }
    LAST_PRESS_MS.store(now_ms, Ordering::Relaxed);
    true
}
