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

//! Identifiers and the handful of MAC byte-registers named by the power
//! sequence. The register offsets are the stable rtw88 MAC map (shared across
//! the RTL8xxx family); the chip-specific power/RF/firmware tables are ported
//! separately from rtw88 with attribution.

mod pci;
pub mod regs;

pub use pci::{PCI_VENDOR_REALTEK, RTL8821CE_DEVICE_IDS};
