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

//! The PCI `_DSM`'s read-only facts (`nonos_aml::pci_dsm`): each host
//! bridge's boot-configuration rule and device readiness durations, and
//! each device's firmware label, read at start, logged, and served to the
//! inventory by PCI_FIRMWARE.

mod find;
mod hosts;
mod say;
mod state;
mod wire;

pub use find::find;
pub use state::PciFirmware;
pub use wire::pci_firmware;
