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

//! Taking the Global Lock for the host's own use (an EC whose _GLK is 1):
//! the same protocol the interpreter follows (ACPI 6.5, 5.2.10.1), bounded,
//! and a no-op on a platform without a FACS. The kernel is told as
//! `lock::word` tells it, so this capsule's exit releases a lock it held.

use super::lock::swap;
use super::state::CapsuleHost;
use crate::events_io::PortIo;
use crate::sys::acpi_event::lock_told;
use crate::sys::clock::sleep_ms;
use nonos_aml::region::{acquire_step, release_step};

/// A second of waiting for firmware to give the lock up.
const TRIES: u32 = 1000;

pub(super) fn take(h: &mut CapsuleHost) -> bool {
    let Some(va) = h.facs else { return true };
    let mut old = 0;
    for _ in 0..TRIES {
        let (new, takes) = acquire_step(old);
        let found = swap(va, old, new);
        if found != old {
            old = found;
            continue;
        }
        if takes {
            h.lock_held = true;
            lock_told(true);
            return true;
        }
        sleep_ms(1);
        old = new;
    }
    false
}

pub(super) fn give(h: &mut CapsuleHost) {
    h.lock_held = false;
    let Some(va) = h.facs else { return };
    lock_told(false);
    let mut old = 2;
    for _ in 0..TRIES {
        let (new, pending) = release_step(old);
        let found = swap(va, old, new);
        if found == old {
            if let (true, Some(cnt)) = (pending, h.pm1_cnt) {
                let _ = cnt.release_lock(&mut PortIo);
            }
            return;
        }
        old = found;
    }
}
