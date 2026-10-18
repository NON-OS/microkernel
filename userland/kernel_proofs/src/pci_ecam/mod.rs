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
 * PCI configuration access: whether an access names a register, which ECAM
 * window reaches a (segment, bus), what the route picks without one, and the
 * registry the windows are published in. The kernel's modules are included
 * by path, in the same tree shape, so their relative imports resolve.
 *
 * Before them there was one global window, the ports aliased an offset of
 * 0x100 onto the first 256 bytes, and x86 never used the MCFG it parsed.
 */

#[cfg(test)]
mod check_tests;
pub mod config;
pub mod constants;
#[path = "../../../../src/drivers/pci/error.rs"]
pub mod error;
pub mod types;
