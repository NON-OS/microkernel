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
 * The fixed port windows the broker gives an IDE function in compatibility
 * mode and VBoxVGA, the ISA line each such channel interrupts on, and the
 * overlap rule that keeps a port to one device's claimant. The kernel's pure
 * files are included by path.
 */

#[path = "../../../../src/hardware/broker/legacy/compat.rs"]
pub mod compat;
#[path = "../../../../src/hardware/broker/legacy/dispi.rs"]
pub mod dispi;
mod dispi_tests;
#[path = "../../../../src/hardware/broker/pio/fixed_line.rs"]
pub mod fixed_line;
mod mode_tests;
#[path = "../../../../src/hardware/broker/pio/overlap.rs"]
pub mod overlap;
mod port_tests;
mod reach_tests;
mod support;
