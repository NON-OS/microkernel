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

//! Leaving the reason, then raising the NMI.

use core::sync::atomic::{fence, AtomicBool, Ordering};

use crate::smp::constants::MAX_CPUS;

/// Set once by the cpu that stops the machine. Every cpu that takes an NMI
/// afterwards halts.
pub(super) static HALT_ALL: AtomicBool = AtomicBool::new(false);

/// One flag per cpu, raised before a shootdown NMI is sent to it.
pub(super) static KICKED: [AtomicBool; MAX_CPUS] = [const { AtomicBool::new(false) }; MAX_CPUS];

/// Send cpu `cpu` (at `apic_id`) an NMI on behalf of the round in flight.
/// `false` means nothing could be sent.
pub(crate) fn kick(cpu: usize, apic_id: u32) -> bool {
    // Only the multi-core image's NMI handler knows what a kick is.
    if !cfg!(feature = "nonos-smp") {
        return false;
    }
    let Some(flag) = KICKED.get(cpu) else {
        return false;
    };
    flag.store(true, Ordering::Release);
    // As for the vector: the ICR write must not overtake the flag.
    fence(Ordering::SeqCst);
    send_one(apic_id)
}

/// Stop every other cpu, masked or not. `false` means nothing could be sent.
pub(crate) fn halt_others() -> bool {
    HALT_ALL.store(true, Ordering::Release);
    fence(Ordering::SeqCst);
    send_others()
}

#[cfg(target_arch = "x86_64")]
fn send_one(apic_id: u32) -> bool {
    crate::arch::x86_64::interrupt::apic::nmi_one(apic_id)
}

#[cfg(target_arch = "x86_64")]
fn send_others() -> bool {
    crate::arch::x86_64::interrupt::apic::nmi_others()
}

/// No NMI command on the other backends; the callers fall back to the vector.
#[cfg(not(target_arch = "x86_64"))]
fn send_one(_apic_id: u32) -> bool {
    false
}

#[cfg(not(target_arch = "x86_64"))]
fn send_others() -> bool {
    false
}
