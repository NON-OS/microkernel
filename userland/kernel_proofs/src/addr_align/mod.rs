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
 * Address rounding lands on a multiple of the alignment.
 *
 * The kernel's physical and virtual address types are included by path.
 * align_down and align_up cleared low bits with !(align - 1), which rounds
 * only for a power of two, so ten aligned down to three gave eight, and an
 * alignment of zero underflowed. The checks below fail against that code.
 */

#[path = "../../../../src/memory/addr/phys.rs"]
pub mod phys;
#[path = "../../../../src/memory/addr/virt.rs"]
pub mod virt;
mod tests;
