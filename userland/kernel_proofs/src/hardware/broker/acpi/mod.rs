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
 * The platform device record: its layout, the decoder, the encoder the
 * ACPI capsule mounts too, and the device-table record built from it;
 * the USB port facts a host's record carries beside it; the domain a
 * memory range or a PCI bus lies in; what each root bridge's _OSC gave up;
 * MkAcpiSlot's wire and masks; who reads the fixed power button, and what
 * the ACPI capsule's exit takes back; the processors' thermal limits.
 */

#[path = "../../../../../../src/hardware/broker/acpi/button_owner.rs"]
pub mod button_owner;
#[path = "../../../../../../src/hardware/broker/acpi/dgon_wire.rs"]
pub mod dgon_wire;
#[path = "../../../../../../src/hardware/broker/acpi/leave_rule.rs"]
pub mod leave_rule;
#[path = "../../../../../../src/hardware/broker/acpi/loaded_rule.rs"]
pub mod loaded_rule;
#[path = "../../../../../../src/hardware/broker/acpi/numa_node.rs"]
pub mod numa_node;
#[path = "../../../../../../src/hardware/broker/acpi/osc_grant.rs"]
pub mod osc_grant;
#[path = "../../../../../../src/hardware/broker/acpi/platform_decode.rs"]
pub mod platform_decode;
#[path = "../../../../../../src/hardware/broker/acpi/platform_encode.rs"]
pub mod platform_encode;
#[path = "../../../../../../src/hardware/broker/acpi/first_pass.rs"]
pub mod first_pass;
#[path = "../../../../../../src/hardware/broker/acpi/first_pass_rule.rs"]
pub mod first_pass_rule;
#[path = "../../../../../../src/hardware/broker/acpi/platform_record.rs"]
pub mod platform_record;
#[path = "../../../../../../src/hardware/broker/acpi/platform_wire.rs"]
pub mod platform_wire;
#[path = "../../../../../../src/hardware/broker/acpi/s5_choice.rs"]
pub mod s5_choice;
#[path = "../../../../../../src/hardware/broker/acpi/same_group.rs"]
pub mod same_group;
#[path = "../../../../../../src/hardware/broker/acpi/slot_rule.rs"]
pub mod slot_rule;
#[path = "../../../../../../src/hardware/broker/acpi/slot_wire.rs"]
pub mod slot_wire;
#[path = "../../../../../../src/hardware/broker/acpi/table_measure.rs"]
pub mod table_measure;
#[path = "../../../../../../src/hardware/broker/acpi/thermal_limit_rule.rs"]
pub mod thermal_limit_rule;
#[path = "../../../../../../src/hardware/broker/acpi/usb_port_wire.rs"]
pub mod usb_port_wire;
