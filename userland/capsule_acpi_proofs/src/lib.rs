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

//! Proofs for capsule_acpi's pure parts, built on the host against the
//! capsule's own sources, included by path.

extern crate alloc;

#[path = "../../capsule_acpi/src/cpu/caps.rs"]
pub mod caps;
#[path = "../../capsule_acpi/src/cpu/caps_kernel.rs"]
pub mod caps_kernel;
#[path = "../../capsule_acpi/src/serial/bus_key.rs"]
pub mod bus_key;
pub mod native;
#[path = "../../capsule_acpi/src/boot/pass.rs"]
pub mod pass;
#[path = "../../capsule_acpi/src/power/off/rule.rs"]
pub mod off_rule;
#[path = "../../capsule_acpi/src/server/policy.rs"]
pub mod policy;
#[path = "props.rs"]
pub mod props;
#[path = "../../capsule_acpi/src/aei/rate.rs"]
pub mod rate;
#[path = "../../capsule_acpi/src/dpower/wake_sweep.rs"]
pub mod wake_sweep;

#[cfg(test)]
mod bus_key_tests;
#[cfg(test)]
mod caps_tests;
#[cfg(test)]
mod native_button_tests;
#[cfg(test)]
mod native_fault_tests;
#[cfg(test)]
mod native_reads;
#[cfg(test)]
mod native_regs_tests;
#[cfg(test)]
mod native_slot_tests;
#[cfg(test)]
mod native_walk_tests;
#[cfg(test)]
mod off_tests;
#[cfg(test)]
mod pass_tests;
#[cfg(test)]
mod policy_table_tests;
#[cfg(test)]
mod policy_tests;
#[cfg(test)]
mod props_tests;
#[cfg(test)]
mod rate_tests;
#[cfg(test)]
mod thread_tests;
#[cfg(test)]
mod wake_model;
#[cfg(test)]
mod wake_sweep_tests;
