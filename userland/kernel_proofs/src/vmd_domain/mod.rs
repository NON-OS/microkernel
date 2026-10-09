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
 * The Intel VMD domain: which devices are VMDs, where their child buses start
 * and sit in CFGBAR, and how the buses and memory behind them are assigned.
 * The kernel's pure module is included by path and driven against a
 * simulated bus that decodes bus numbers and BAR sizing as hardware does.
 *
 * Before it the kernel had no VMD support at all: with RST on in firmware the
 * NVMe drive behind the VMD never appeared and no disk driver started.
 */

#[path = "../../../../src/drivers/pci/vmd/domain/mod.rs"]
pub mod domain;
#[path = "../../../../src/hardware/inventory/vmd.rs"]
pub mod inventory_vmd;
mod ids_tests;
mod sim;
mod tests;
