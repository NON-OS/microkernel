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
 * MkLocalSign and MkLocalVerify hash the caller's image where it lies, a
 * bounded chunk at a time, instead of copying it whole into the kernel heap.
 * The kernel's own file, and the two calls' use of it.
 */

#[path = "../../../../src/syscall/microkernel/local_image/digest.rs"]
pub mod digest;
#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod source_tests;
#[cfg(test)]
mod tests;
