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
 * MkPciConfigWrite's two forms (L37): the plain one ORs a Command value of
 * writable bits on, the masked one names the bits it changes so a driver
 * can turn Bus Master or Interrupt Disable off. The kernel's pure rules are
 * included by path.
 */

#[path = "../../../../src/hardware/broker/pci/change.rs"]
pub mod change;
#[path = "../../../../src/hardware/broker/pci/command_bits.rs"]
pub mod command_bits;
#[cfg(test)]
mod refusal_tests;
#[cfg(test)]
mod tests;
