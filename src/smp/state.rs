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

use super::constants::MAX_CPUS;
use super::types::CpuDescriptor;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

pub(crate) static CPU_DESCRIPTORS: [CpuDescriptor; MAX_CPUS] = {
    const INIT: CpuDescriptor = CpuDescriptor::new();
    [INIT; MAX_CPUS]
};

pub(super) static CPU_COUNT: AtomicUsize = AtomicUsize::new(1);

pub(super) static CPUS_ONLINE: AtomicUsize = AtomicUsize::new(1);

pub(super) static BSP_APIC_ID: AtomicU32 = AtomicU32::new(0);

pub(super) static SMP_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) static BSP_INITIALIZING: AtomicBool = AtomicBool::new(false);

pub(super) static AP_STARTUP_BARRIER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn cpu_count() -> usize {
    CPU_COUNT.load(Ordering::Acquire)
}

/// How many CPUs are running. This is a population count and never an index
/// bound: cpu numbers are handed out once per AP that is attempted and are not
/// reused when one fails to come up, so with any failed AP the live numbers
/// are sparse and the largest of them is greater than this. Walking `0..this`
/// would then miss a running CPU and visit a slot that never started. Use
/// [`cpu_is_online`] over `0..MAX_CPUS` to enumerate.
pub(crate) fn cpus_online() -> usize {
    CPUS_ONLINE.load(Ordering::Acquire)
}

/// Whether `cpu` is a CPU that came up and is running.
///
/// An AP that missed its start deadline is left `Offline` by `ap_unit::start`,
/// and a slot that was never attempted has never left its initial state, so
/// this is the only safe way to decide whether a cpu number can be expected to
/// answer an IPI.
pub(crate) fn cpu_is_online(cpu: usize) -> bool {
    cpu < MAX_CPUS && CPU_DESCRIPTORS[cpu].state() == super::types::CpuState::Online
}
