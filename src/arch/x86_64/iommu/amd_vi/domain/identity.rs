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

//! How many device table entries still let a device reach all of memory.
//! Bring-up writes none: every id starts blocked, or in a domain of its
//! reserved memory (K4), and a release leaves it denied. Every write is
//! counted anyway, so the posture line says what the table holds, never
//! what bring-up meant it to.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::super::dte::Dte;
use super::super::identity_change::identity_change;

static IDENTITY: AtomicUsize = AtomicUsize::new(0);

/// Called after every device table write, with the entry it replaced.
pub(super) fn note(before: Dte, after: Dte) {
    match identity_change(before, after) {
        1 => {
            IDENTITY.fetch_add(1, Ordering::AcqRel);
        }
        -1 => {
            // Never below zero, so a write the count missed cannot wrap it.
            let _ = IDENTITY
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| Some(n.saturating_sub(1)));
        }
        _ => {}
    }
}

pub fn identity_devices() -> usize {
    IDENTITY.load(Ordering::Acquire)
}
