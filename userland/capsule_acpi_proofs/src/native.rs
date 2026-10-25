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

//! capsule_acpi's native hot-plug slot, its pure parts, by path, and the
//! kernel's MkAcpiSlot rule it writes through.

#[path = "../../capsule_acpi/src/hotplug/native/fault.rs"]
pub mod fault;
#[path = "../../capsule_acpi/src/hotplug/native/read.rs"]
pub mod read;
#[path = "../../capsule_acpi/src/hotplug/native/regs.rs"]
pub mod regs;
#[path = "../../capsule_acpi/src/hotplug/native/slot.rs"]
pub mod slot;
#[allow(dead_code)]
#[path = "../../../src/hardware/broker/acpi/slot_rule.rs"]
pub mod slot_rule;
#[allow(dead_code)]
#[path = "../../../src/hardware/broker/acpi/slot_wire.rs"]
pub mod slot_wire;
#[path = "../../capsule_acpi/src/hotplug/native/step.rs"]
pub mod step;
#[path = "../../capsule_acpi/src/hotplug/native/step_timed.rs"]
pub mod step_timed;
#[path = "../../capsule_acpi/src/hotplug/native/times.rs"]
pub mod times;
