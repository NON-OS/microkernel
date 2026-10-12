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
 * The platform events the ACPI capsule reports through MkAcpiEvent: their
 * decoding, the critical thermal watch, and the lid, included by path as
 * the kernel builds them.
 */

#[path = "../../../../src/hardware/broker/acpi/battery_watch.rs"]
pub mod battery_watch;
#[path = "../../../../src/hardware/broker/acpi/critical_watch.rs"]
pub mod critical_watch;
#[path = "../../../../src/hardware/broker/acpi/lid_state.rs"]
pub mod lid_state;
#[path = "../../../../src/hardware/broker/acpi/platform_event.rs"]
pub mod platform_event;
#[path = "../../../../src/hardware/broker/acpi/thermal_watch.rs"]
pub mod thermal_watch;

#[cfg(test)]
mod tests;
