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

//! The ACPI capsule leaving, on x86 (F-107): the events it armed disarmed
//! through the port bus, and the Global Lock it said it held released
//! where firmware shares it. `hw::leave_events` decides; this reaches the
//! hardware.

use core::sync::atomic::{AtomicU32, Ordering};

use super::hw::leave_events::{disarm, global_release, release_word};
use super::hw::port_bus::PortBus;
use super::parser::{self, phys::directmap};

/// The Global Lock's offset in the FACS (ACPI 6.5, table 5.14).
const LOCK_OFFSET: u64 = 16;
/// Looks at the word before giving up; firmware changes it only with locked
/// operations, so a few suffice. Our bound, from no source.
const TRIES: u32 = 64;

pub fn events(release_lock: bool) {
    let Some(fadt) = parser::with_data(|d| d.fadt).flatten() else { return };
    disarm(&mut PortBus, &fadt);
    if release_lock && fadt.firmware_ctrl != 0 && lock_released(fadt.firmware_ctrl) {
        global_release(&mut PortBus, &fadt);
    }
}

/// True when firmware was waiting for the lock.
fn lock_released(facs: u64) -> bool {
    let Some(va) = facs.checked_add(LOCK_OFFSET).and_then(directmap) else { return false };
    // SAFETY: the FACS is firmware memory inside the direct map (the table
    // copies read it the same way) and 64-byte aligned (5.2.10), so the
    // word is four-byte aligned; firmware changes it only atomically.
    let word = unsafe { &*(va as *const AtomicU32) };
    let mut old = word.load(Ordering::SeqCst);
    for _ in 0..TRIES {
        let (new, pending) = release_word(old);
        match word.compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return pending,
            Err(found) => old = found,
        }
    }
    false
}
