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


//! SPCR and DBG2 on a PC: the one parser of each, mounted from the aarch64
//! tables by path (byte parsers with nothing Arm in them), with the header
//! check and field readers they share.

#[path = "../../../../aarch64/acpi/dbg2.rs"]
pub mod dbg2;
#[path = "../../../../aarch64/acpi/dbg2_device.rs"]
pub mod dbg2_device;
#[path = "../../../../aarch64/acpi/le.rs"]
pub mod le;
#[path = "../../../../aarch64/acpi/sdt.rs"]
pub mod sdt;
#[path = "../../../../aarch64/acpi/spcr.rs"]
pub mod spcr;
#[path = "../../../../aarch64/acpi/spcr_fields.rs"]
pub mod spcr_fields;
#[path = "../../../../aarch64/acpi/spcr_tail.rs"]
pub mod spcr_tail;
