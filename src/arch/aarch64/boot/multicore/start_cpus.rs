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

use core::sync::atomic::Ordering;

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::boot::stack::get_kernel_stack;
use crate::sys::serial;

use super::state::CPUS_ONLINE;
use super::{roster, start_one};

/// How long to wait for a released CPU to check in before giving up on it.
const CHECKIN_SPINS: u64 = 200_000_000;

/// Power on every CPU the device tree listed except the one running this.
///
/// A CPU that will not start is reported and skipped rather than halting the
/// machine: the boot CPU is enough to run the system, and taking the whole
/// kernel down because one core of many refused to come up turns a degraded
/// boot into no boot at all.
pub fn start_secondary_cpus(boot_info: &BootInfo) {
    crate::arch::aarch64::mmu::publish_for_secondaries();
    let mut released = 0u32;

    for index in 1..roster::len() {
        let Some(affinity) = roster::affinity_of(index) else {
            continue;
        };
        let Some(stack_top) = get_kernel_stack(index) else {
            serial::println(b"[SMP] no stack for secondary, skipping");
            continue;
        };
        match start_one::start(boot_info, index, affinity, stack_top) {
            Ok(()) => released += 1,
            Err(reason) => {
                serial::print(b"[SMP] a secondary did not start: ");
                serial::println(reason);
            }
        }
    }

    wait_for_checkin(released + 1);
}

fn wait_for_checkin(expected: u32) {
    let mut spins = 0u64;
    while CPUS_ONLINE.load(Ordering::Acquire) < expected {
        if spins >= CHECKIN_SPINS {
            serial::println(b"[SMP] timed out waiting for a secondary to check in");
            return;
        }
        spins += 1;
        core::hint::spin_loop();
    }
}
