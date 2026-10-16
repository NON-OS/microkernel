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

//! capsule_acpi's native hot-plug slot, its pure parts, by path.

#[path = "../../capsule_acpi/src/hotplug/native/read.rs"]
pub mod read;
#[path = "../../capsule_acpi/src/hotplug/native/regs.rs"]
pub mod regs;
#[path = "../../capsule_acpi/src/hotplug/native/slot.rs"]
pub mod slot;
#[path = "../../capsule_acpi/src/hotplug/native/step.rs"]
pub mod step;
#[path = "../../capsule_acpi/src/hotplug/native/times.rs"]
pub mod times;
