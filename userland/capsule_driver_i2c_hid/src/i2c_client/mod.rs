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

mod acpi_hid;
pub mod budget;
mod doorbell;
pub mod gate;
mod seq;
mod service;
pub mod transfer;
mod wire;

pub use acpi_hid::query_acpi_hid;
pub use doorbell::query_doorbell;
pub use service::resolve;
pub use transfer::write_read;
