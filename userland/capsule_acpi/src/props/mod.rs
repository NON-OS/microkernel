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

//! PROPERTIES (12): a device's _DSD properties and subnodes, read by
//! `nonos_aml::props`, for the driver that holds the device. The wire
//! (`wire`, `wire_write`, `wire_read`, `wire_cursor`) is pure and mounted
//! by `capsule_acpi_proofs`. The reading half (`wire_read`, `wire_cursor`)
//! is not compiled here: a driver mounts those files to read the reply.

mod convert;
mod serve;
mod wire;
mod wire_write;

pub use serve::properties;
