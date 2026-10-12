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

//! The platform device record as the kernel decodes it: its layout, the
//! decoder and the encoder, mounted from the kernel's broker by path so the
//! capsule cannot send a record the kernel reads differently.
//! `kernel_proofs` proves the round trip over these same files.

#[path = "../../../../../src/hardware/broker/acpi/platform_decode.rs"]
mod platform_decode;
#[path = "../../../../../src/hardware/broker/acpi/platform_encode.rs"]
mod platform_encode;
#[path = "../../../../../src/hardware/broker/acpi/platform_wire.rs"]
#[allow(dead_code)] // the record's size bound is the kernel's to check
mod platform_wire;

pub use platform_encode::encode;
pub use platform_wire::{Platform, GPIO, HAS_IRQ, I2C, MAX_WINDOWS, SD};
