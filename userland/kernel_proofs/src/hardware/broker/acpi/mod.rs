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
 * ACPI capsule mounts too, and the device-table record built from it.
 */

#[path = "../../../../../../src/hardware/broker/acpi/platform_decode.rs"]
pub mod platform_decode;
#[path = "../../../../../../src/hardware/broker/acpi/platform_encode.rs"]
pub mod platform_encode;
#[path = "../../../../../../src/hardware/broker/acpi/platform_record.rs"]
pub mod platform_record;
#[path = "../../../../../../src/hardware/broker/acpi/platform_wire.rs"]
pub mod platform_wire;
