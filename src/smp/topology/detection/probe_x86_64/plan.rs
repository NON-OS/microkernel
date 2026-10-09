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

//! Which MADT processors become APs, decided apart from where the list came
//! from so the host proofs can feed it real firmware shapes.
//!
//! The list is firmware's, and real firmware is untidy in ways an emulator is
//! not. Each rule below answers one of them:
//!
//! - Disabled entries are skipped. An entry that is only online capable is a
//!   hotplug slot, and this kernel does not hotplug, so it is skipped too.
//! - A Local APIC entry with id 0xFF and an x2APIC entry with id 0xFFFFFFFF
//!   are reserved ids, not processors.
//! - The same id listed twice (a Local APIC entry and an x2APIC entry for one
//!   CPU) is started once. Starting it twice sends INIT to a CPU that is
//!   already running, which resets it under the kernel.
//! - The boot CPU is never an AP, whether or not firmware lists it.
//! - In xAPIC mode an id above 0xFE cannot be addressed; it is reported and
//!   left off rather than truncated into some other CPU's id.
//! - Past `max_cpus` the rest are reported and left off.
//!
//! APIC ids are not cpu numbers. A hybrid part lists ids like 0, 8, 16, 32,
//! 33 ..., and every one of them is passed through as given.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MadtCpu {
    pub(crate) apic_id: u32,
    pub(crate) enabled: bool,
    pub(crate) x2apic_entry: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlanCounts {
    pub(crate) accepted: usize,
    pub(crate) disabled: usize,
    pub(crate) invalid: usize,
    pub(crate) duplicate: usize,
    pub(crate) unaddressable: usize,
    pub(crate) over_limit: usize,
}

/// Walk `entries` and hand each AP to start to `accept`, in MADT order.
/// `max_cpus` counts the boot CPU, so at most `max_cpus - 1` are accepted.
pub(crate) fn plan(
    entries: &[MadtCpu],
    own_apic_id: u32,
    x2apic_mode: bool,
    max_cpus: usize,
    mut accept: impl FnMut(u32),
) -> PlanCounts {
    let mut counts = PlanCounts::default();
    for (i, entry) in entries.iter().enumerate() {
        if !entry.enabled {
            counts.disabled += 1;
            continue;
        }
        let placeholder = if entry.x2apic_entry { u32::MAX } else { 0xFF };
        if entry.apic_id == placeholder {
            counts.invalid += 1;
            continue;
        }
        if entry.apic_id == own_apic_id {
            continue;
        }
        if entries[..i].iter().any(|e| e.enabled && e.apic_id == entry.apic_id) {
            counts.duplicate += 1;
            continue;
        }
        if !x2apic_mode && entry.apic_id > 0xFE {
            counts.unaddressable += 1;
            continue;
        }
        if counts.accepted + 1 >= max_cpus {
            counts.over_limit += 1;
            continue;
        }
        counts.accepted += 1;
        accept(entry.apic_id);
    }
    counts
}
