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

/* What setup turned off, held for the rest of the boot. */

use core::sync::atomic::{AtomicU32, Ordering};

static OFF: AtomicU32 = AtomicU32::new(0);

/*
 * Record what setup turned off, of the apps this kernel carries. It only
 * ever adds: nothing turns an app back on before the next boot.
 */
#[cfg(feature = "microkernel-setup-wizard")]
pub(crate) fn choose(off: u8) {
    let off = off as u32 & super::PRESENT;
    OFF.fetch_or(off, Ordering::SeqCst);
    if off != 0 {
        crate::sys::serial::print(b"[INIT] apps turned off at setup, mask ");
        crate::sys::serial::print_hex(off as u64);
        crate::sys::serial::println(b"");
    }
}

/* Apps the boot profile does not run, recorded before any app spawns. */
pub(super) fn withhold(mask: u32) {
    OFF.fetch_or(mask & super::PRESENT, Ordering::SeqCst);
}

pub(crate) fn off() -> u32 {
    OFF.load(Ordering::SeqCst)
}

pub(crate) fn is_off(bit: u32) -> bool {
    off() & bit != 0
}
