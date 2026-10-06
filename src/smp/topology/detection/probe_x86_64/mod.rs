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

//! CPU topology from CPUID, corrected by the ACPI MADT.
//!
//! CPUID describes what one package can do; the MADT lists what the firmware
//! actually enabled, which is the smaller and truer number on a machine with
//! cores disabled or a socket unpopulated.

extern crate alloc;

mod leaves;
mod plan;

use alloc::vec::Vec;

use super::super::types::CpuTopology;
use super::state::{set_ap_list, set_topology};
use crate::smp::MAX_CPUS;
use leaves::{cpuid, via_leaf_04, via_leaf_0b};

pub(super) fn detect() -> usize {
    let mut topology = CpuTopology {
        logical_cpus: 1,
        physical_cores: 1,
        numa_nodes: 1,
        hyperthreading: false,
        x2apic: false,
    };

    let (max_basic, ..) = cpuid(0, 0);
    let (_, _, ecx, edx) = cpuid(1, 0);
    topology.x2apic = ecx & (1 << 21) != 0;
    topology.hyperthreading = edx & (1 << 28) != 0;

    topology.logical_cpus = if max_basic >= 0x0B {
        via_leaf_0b(&mut topology)
    } else if max_basic >= 0x04 {
        via_leaf_04(&mut topology)
    } else if max_basic >= 0x01 {
        (((cpuid(1, 0).1) >> 16) & 0xFF).max(1) as usize
    } else {
        1
    };

    // The count is what will actually be started, plus this CPU: the MADT's
    // enabled entries after the planner has dropped duplicates, placeholders,
    // unaddressable ids and anything past MAX_CPUS. CPUID's count is only the
    // answer on a machine with no MADT.
    let aps = secondaries(topology.logical_cpus);
    topology.logical_cpus = (aps.len() + 1).min(MAX_CPUS);

    set_topology(topology);
    set_ap_list(aps);
    topology.logical_cpus
}

/// Every processor the MADT says to start, by APIC id, except this one.
/// Without a MADT the APIC IDs are assumed dense from zero, which is what a
/// machine old enough to lack one does.
fn secondaries(logical_cpus: usize) -> Vec<u32> {
    let own = crate::arch::interrupt_controller::local_id();
    let processors = crate::arch::x86_64::acpi::processors();
    if processors.is_empty() {
        return (0..logical_cpus.min(MAX_CPUS) as u32).filter(|id| *id != own).collect();
    }
    let entries: Vec<plan::MadtCpu> = processors
        .iter()
        .map(|p| plan::MadtCpu {
            apic_id: p.apic_id,
            enabled: p.enabled,
            x2apic_entry: p.is_x2apic,
        })
        .collect();
    let x2apic_mode = crate::sys::apic::lapic_state().is_some_and(|(x2, _)| x2);
    let mut aps = Vec::new();
    let counts = plan::plan(&entries, own, x2apic_mode, MAX_CPUS, |id| aps.push(id));

    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] madt aps=").dec(counts.accepted as u64);
    l.str(b" disabled=").dec(counts.disabled as u64);
    l.str(b" duplicate=").dec(counts.duplicate as u64);
    l.str(b" invalid=").dec(counts.invalid as u64);
    l.str(b" unaddressable=").dec(counts.unaddressable as u64);
    l.str(b" over_limit=").dec(counts.over_limit as u64);
    l.end();
    if counts.over_limit > 0 {
        crate::log_warn!(
            "[SMP] {} CPUs past MAX_CPUS={} left offline",
            counts.over_limit,
            MAX_CPUS
        );
    }
    if counts.unaddressable > 0 {
        crate::log_warn!(
            "[SMP] {} CPUs have APIC ids above 0xFE and the APIC is in xAPIC mode; left offline",
            counts.unaddressable
        );
    }
    aps
}
