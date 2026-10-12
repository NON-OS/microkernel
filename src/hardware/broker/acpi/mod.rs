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

pub mod battery_watch;
pub mod claim_holder;
pub mod cpu_layout;
pub mod cpu_record;
pub mod cpu_store;
pub mod cpu_take;
pub mod cpu_wire;
pub mod critical_watch;
pub mod device_holder;
mod ecam_note;
mod error;
pub mod external_port;
mod granting;
pub mod hw_errors;
mod io_grant;
mod kernel;
pub mod late_bars;
pub mod lid_state;
mod memory_grant;
mod memory_record;
mod pci_access;
mod pci_admit;
pub mod platform_add;
pub mod platform_decode;
pub mod platform_encode;
pub mod platform_event;
pub mod platform_paths;
pub mod platform_record;
pub mod platform_rule;
pub mod platform_table;
pub mod platform_wire;
pub mod same_group;
mod port_access;
pub mod power_key;
pub mod power_report;
pub mod probe_function;
mod report;
pub mod reserved_publish;
pub mod rule;
mod sci;
pub mod sleep_adopt;
pub mod sleep_request;
pub mod smbios;
pub mod smbios_entry;
mod table_allow;
mod table_check;
mod table_report;
mod tables;
pub mod thermal_watch;
pub mod usb_port_wire;
pub mod usb_ports;

pub use error::RegionError;
pub use io_grant::grant_io;
pub use memory_grant::grant_memory;
pub use memory_record::{MemoryGrant, ACPI_DEVICE};
pub use pci_access::{pci_read, pci_write};
pub use port_access::{port_read, port_write};
pub use sci::{register_sci, PNP_DEVICE_ACPI, PNP_VENDOR};
pub use table_check::TableError;
pub use table_report::table_for_capsule;
