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

//! L55: the ACPI capsule's first publish pass is done. Init waits on it
//! before it spawns for the device families the capsule publishes, so a
//! host only ACPI names (an SD or eMMC host, a GPIO controller) is in the
//! table before its driver is asked for. One flag, set once and never
//! cleared: a capsule that restarts can never make init wait again.
//! Pure: `kernel_proofs` mounts it.

use core::sync::atomic::{AtomicBool, Ordering};

static DONE: AtomicBool = AtomicBool::new(false);

/// Sets the flag; true only the first time.
pub fn mark() -> bool {
    !DONE.swap(true, Ordering::AcqRel)
}

/// Whether the first pass is done.
pub fn done() -> bool {
    DONE.load(Ordering::Acquire)
}
