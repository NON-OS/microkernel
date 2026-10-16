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

mod external;
mod init;
mod list;
mod numa;
mod pci_record;
mod remove;
mod state;

// For the ACPI broker, which only x86 builds.
#[allow(unused_imports)]
pub(in crate::hardware::broker) use init::add_pci_function;
pub use external::external_port_of;
#[allow(unused_imports)] // x86 alone declares ports yet
pub(in crate::hardware::broker) use external::mark as mark_external;
pub use init::init_from_pci;
#[allow(unused_imports)] // x86 alone knows domains yet
pub(in crate::hardware::broker) use numa::set_numa_nodes;
pub(super) use init::register_platform_device;
pub(super) use list::class_of;
#[allow(unused_imports)] // the ACPI broker's alone, which only x86 builds
pub(in crate::hardware::broker) use remove::remove;
pub use list::{contains, list, list_by_class};
