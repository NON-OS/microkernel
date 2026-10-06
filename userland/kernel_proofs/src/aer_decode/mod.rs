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
 * The PCIe AER status bits the boot report names, and the extended
 * capability header it walks. The kernel source is included by path and held
 * against PCIe Base 5.0, 7.6.3 and 7.8.4.
 */

#[path = "../../../../src/bus/pci/aer/decode.rs"]
pub mod decode;
mod tests;
