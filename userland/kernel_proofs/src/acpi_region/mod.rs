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
 * Which OperationRegion windows the ACPI capsule may be granted. The rule is
 * included by path, the directory as the kernel builds it.
 */

// The record is mounted once, with the broker's device types.
pub use crate::hardware::broker::acpi::{platform_decode, platform_wire};
#[path = "../../../../src/hardware/broker/acpi/platform_rule.rs"]
pub mod platform_rule;
#[path = "../../../../src/hardware/broker/acpi/rule/mod.rs"]
pub mod rule;

#[cfg(test)]
mod aml_ports_tests;
#[cfg(test)]
mod machine;
#[cfg(test)]
mod model_facts;
#[cfg(test)]
mod platform_admit_tests;
#[cfg(test)]
mod platform_tests;
#[cfg(test)]
mod seeded_gen;
#[cfg(test)]
mod seeded_tests;
#[cfg(test)]
mod sweep;
#[cfg(test)]
mod smbios_span_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_bridge;
#[cfg(test)]
mod tests_bridge_log;
#[cfg(test)]
mod tests_ports;
#[cfg(test)]
mod tests_power;
#[cfg(test)]
mod tests_sleep_enable;
#[cfg(test)]
mod sleep_enable_glue_tests;
