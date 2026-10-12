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

//! The ACPI capsule's grants: the tables it reads and the OperationRegion
//! windows its interpreter may reach. `rule`, `table_check` and
//! `table_allow` are pure and proven in `kernel_proofs`; the rest supplies
//! them this machine's facts and does what they allow.

mod error;
mod granting;
mod io_grant;
mod kernel;
mod memory_grant;
mod memory_record;
mod pci_access;
mod pci_admit;
mod port_access;
mod report;
pub mod rule;
mod sci;
mod table_allow;
mod table_check;
mod table_report;
mod tables;

pub use error::RegionError;
pub use io_grant::grant_io;
pub use memory_grant::grant_memory;
pub use memory_record::{MemoryGrant, ACPI_DEVICE};
pub use pci_access::{pci_read, pci_write};
pub use port_access::{port_read, port_write};
pub use sci::{register_sci, PNP_DEVICE_ACPI, PNP_VENDOR};
pub use table_check::TableError;
pub use table_report::table_for_capsule;
