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

//! Host proofs, group 8 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

/// The IPC receives check what they write to before they wait.
#[path = "handover_ipc_tests.rs"]
mod handover_ipc_tests;
/// Late versus starved for an ipc.call with no answer (B-04).
#[path = "ipc_death/mod.rs"]
pub mod ipc_death;
#[path = "ipc_silence/mod.rs"]
pub mod ipc_silence;
#[path = "ipc_call_verdict/mod.rs"]
pub mod ipc_call_verdict;
#[path = "legacy_ports/mod.rs"]
pub mod legacy_ports;
#[path = "load_average/mod.rs"]
pub mod load_average;
#[path = "mmio_memory_type/mod.rs"]
pub mod mmio_memory_type;
#[path = "pci_bar_slots/mod.rs"]
pub mod pci_bar_slots;
#[path = "pci_config_change/mod.rs"]
pub mod pci_config_change;
#[path = "registry_names/mod.rs"]
pub mod registry_names;
// CPUID is x86_64's alone, so its proofs are too.
#[path = "block_slow/mod.rs"]
pub mod block_slow;
#[cfg(target_arch = "x86_64")]
#[path = "spectre_cpuid/mod.rs"]
pub mod spectre_cpuid;
#[path = "speculation_policy/mod.rs"]
pub mod speculation_policy;
#[path = "surface_grants/mod.rs"]
pub mod surface_grants;
#[path = "surface_register/mod.rs"]
pub mod surface_register;
#[path = "surface_shares/mod.rs"]
pub mod surface_shares;
#[path = "syscall_fmask/mod.rs"]
pub mod syscall_fmask;
#[path = "transport_takeover/mod.rs"]
pub mod transport_takeover;
#[path = "usable_runs/mod.rs"]
pub mod usable_runs;
#[path = "virtio_rng_ring/mod.rs"]
pub mod virtio_rng_ring;
