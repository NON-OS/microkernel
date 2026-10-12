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
 * The broker's device record, its class ids, the platform device record,
 * the BAR a mapping may name, the buses behind an external-facing port
 * and the ranges firmware reserves, by path as the kernel builds them.
 */

pub mod acpi;
#[path = "../../../../../src/hardware/broker/class.rs"]
#[allow(dead_code)] // the proofs read only the ids a platform device takes
pub mod class;
pub mod device;
#[path = "../../../../../src/hardware/broker/external_range.rs"]
pub mod external_range;
pub mod mmio;
pub mod reserved;

#[cfg(test)]
mod bars_tests;
#[cfg(test)]
mod external_tests;
#[cfg(test)]
mod numa_tests;
#[cfg(test)]
mod osc_grant_tests;
#[cfg(test)]
mod round_trip_tests;
#[cfg(test)]
mod same_group_tests;
#[cfg(test)]
mod usb_port_tests;
