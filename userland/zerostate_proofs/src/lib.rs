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

//! Host proofs for zero-state. The kernel's keep rule is mounted from its
//! source, so what is proved here is what the kernel runs.

// The rules the store write and the policy push ask.
#[allow(dead_code)]
#[path = "../../../src/security/zerostate/keep_rule.rs"]
pub mod keep_rule;

// A dying address space's leaf frames, and the names the wipe clears.
#[allow(dead_code)]
#[path = "../../../src/memory/paging/manager/address_space/leaf_release.rs"]
pub mod leaf_release;
#[allow(dead_code)]
#[path = "../../../src/sys/policy/hostname/names_clear.rs"]
pub mod names_clear;

pub mod sweep;

#[cfg(test)]
mod arm_tests;
#[cfg(test)]
mod keep_tests;
#[cfg(test)]
mod leaf_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod rule_tests;
