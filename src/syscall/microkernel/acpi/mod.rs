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

//! The ACPI capsule's calls: its firmware tables and the OperationRegion
//! windows the broker's rule grants it. x86_64 only, as ACPI is here.

mod dispatch;
mod errno;
mod pci;
mod port;
mod power;
mod region;
mod table;

pub use dispatch::handle;
pub use pci::sys_acpi_pci;
pub use port::sys_acpi_port;
pub use power::sys_acpi_power;
pub use region::sys_acpi_region;
pub use table::sys_acpi_table;
