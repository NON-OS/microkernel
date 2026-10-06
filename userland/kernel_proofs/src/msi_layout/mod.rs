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

/*
 * Where the broker writes a function's MSI message, and the control words it
 * writes. The real kernel source is included by path and held against the
 * register offsets of PCI Local Bus 3.0, 6.8.1, and against the capability
 * layouts lspci reports for parts that offer MSI and no MSI-X.
 */

#[path = "../../../../src/hardware/broker/irq/bind/msi_layout.rs"]
pub mod layout;
mod tests;
