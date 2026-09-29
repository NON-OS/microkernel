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
 * The IDT error-code table must name every vector that pushes an error code.
 *
 * The kernel's vectors.rs is included by path. It omitted VMM communication
 * (29, #VC) and security exception (30, #SX), both of which push an error
 * code, so a stub built from it would take that code for the return address.
 */

#[path = "../../../../src/interrupts/idt/vectors.rs"]
pub mod vectors;
mod tests;
