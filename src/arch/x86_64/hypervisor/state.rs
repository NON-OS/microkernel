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

/*
 * The answer, kept for the life of the boot. CPUID gives a first answer to
 * anyone who asks before the PCI table exists; `report` replaces it with the
 * refined one once, after the broker has its devices.
 */

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use super::classify::classify;
use super::guest::Guest;
use super::probe;
use super::vendor::Hypervisor;

const UNPROBED: u8 = 0xFF;

static VENDOR: AtomicU8 = AtomicU8::new(UNPROBED);
static HYPERV_ABI: AtomicBool = AtomicBool::new(false);

fn ensure() {
    if VENDOR.load(Ordering::Acquire) != UNPROBED {
        return;
    }
    let g = classify(&probe::read());
    HYPERV_ABI.store(g.hyperv_abi, Ordering::Relaxed);
    // A racing first caller computes the same words; whoever loses the
    // exchange keeps the winner's answer, so the two can never disagree.
    let _ = VENDOR.compare_exchange(UNPROBED, g.vendor as u8, Ordering::AcqRel, Ordering::Acquire);
}

pub(super) fn store(g: Guest) {
    HYPERV_ABI.store(g.hyperv_abi, Ordering::Relaxed);
    VENDOR.store(g.vendor as u8, Ordering::Release);
}

pub(super) fn guest() -> Guest {
    ensure();
    Guest { vendor: current(), hyperv_abi: hyperv_abi() }
}

/// The hypervisor this boot runs under.
pub fn current() -> Hypervisor {
    ensure();
    Hypervisor::from_code(VENDOR.load(Ordering::Acquire))
}

/// Whether the Hyper-V hypercall interface is on offer, whatever the vendor.
pub fn hyperv_abi() -> bool {
    ensure();
    HYPERV_ABI.load(Ordering::Relaxed)
}
