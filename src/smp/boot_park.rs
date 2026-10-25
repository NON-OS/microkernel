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

//! Parking an AP that claimed its start and then missed its online deadline.
//!
//! The boot CPU moves the AP's claim word (`boot_claim`) from `ENTERED` to
//! `PARK_ASKED`. The AP, at a checkpoint between the steps of its bring-up,
//! where it holds no lock, moves it on to `PARKED` and halts with interrupts
//! masked. Only on seeing `PARKED` does the boot CPU send INIT: an INIT that
//! landed inside a step could reset the core with a lock held, and every later
//! taker would spin. An AP that never reaches a checkpoint never answers, and
//! stays a CPU the rest of the machine must account for.
//!
//! Host-checked by path from `kernel_proofs`; it uses nothing but core.

use core::sync::atomic::Ordering;

use super::boot_claim::{BootClaim, ENTERED, PARKED, PARK_ASKED};

impl BootClaim {
    /// The boot CPU's request that an AP past its online deadline park. False
    /// when the AP never entered, or was already asked.
    pub fn ask_park(&self) -> bool {
        self.0.compare_exchange(ENTERED, PARK_ASKED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    /// Read by the AP at each checkpoint of its bring-up.
    pub fn park_asked(&self) -> bool {
        self.0.load(Ordering::Acquire) == PARK_ASKED
    }

    /// The AP's answer, taken at a checkpoint, just before it halts. True when
    /// it was asked; the AP halts either way once it has seen the request.
    pub fn ap_park(&self) -> bool {
        self.0.compare_exchange(PARK_ASKED, PARKED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    /// The AP has parked: halted, interrupts masked, holding no lock. Only now
    /// may the boot CPU send it INIT.
    pub fn parked(&self) -> bool {
        self.0.load(Ordering::Acquire) == PARKED
    }
}
