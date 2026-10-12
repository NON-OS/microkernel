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

//! Bringing the platform up from the namespace (ACPI 6.5, chapter 6): the
//! _INI walk, \_PIC, _OSC, the device inventory, PCI interrupt routing, and
//! the PCI function a PCI_Config region reaches. Everything runs through
//! the caller's evaluator.

mod eisa;
mod eval;
mod ids;
mod ini;
mod ini_report;
mod inventory;
mod osc;
mod osc_status;
mod osc_uuid;
mod pci_path;
mod pic;
mod prt;
mod prt_link;
mod resource;
mod route;
mod sta;
mod walk;

pub use eisa::eisa_id;
pub use eval::{integer, Eval};
pub use ids::{compatible_ids, hardware_id, HardwareId};
pub use ini::initialize;
pub use ini_report::IniReport;
pub use inventory::{inventory, DeviceInfo};
pub use osc::{negotiate, PCI_HOST_UUID, PLATFORM_UUID};
pub use osc_status::{OscError, MASKED, QUERY};
pub use pci_path::{function_of, SECONDARY_BUS};
pub use pic::{set_pic, PIC_8259, PIC_APIC};
pub use prt::{routes, Route};
pub use resource::interrupts;
pub use sta::{sta, BATTERY, DEFAULT, ENABLED, FUNCTIONING, PRESENT, SHOWN};
