// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Proofs for capsule_acpi's pure parts, on the host, against its own sources.

extern crate alloc;

#[path = "../../capsule_acpi/src/serial/bus_key.rs"]
pub mod bus_key;
#[path = "../../capsule_acpi/src/cpu/caps.rs"]
pub mod caps;
#[path = "../../capsule_acpi/src/cpu/caps_kernel.rs"]
pub mod caps_kernel;
#[path = "../../capsule_acpi/src/sys/line_frame.rs"]
pub mod line_frame;
#[cfg(test)]
mod line_frame_tests;
pub mod native;
#[path = "../../capsule_acpi/src/power/off/rule.rs"]
pub mod off_rule;
#[path = "../../capsule_acpi/src/boot/pass.rs"]
pub mod pass;
#[path = "../../capsule_acpi/src/server/policy.rs"]
pub mod policy;
#[path = "props.rs"]
pub mod props;
#[path = "../../capsule_acpi/src/aei/rate.rs"]
pub mod rate;
pub mod release;
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
