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
 * Which process owns which address space, read without the paging manager
 * lock by every context switch and system call: the kernel's own files,
 * one writer at a time, readers never seeing half a write.
 */

#[cfg(test)]
mod fill_tests;
#[cfg(test)]
mod race_tests;
#[path = "../../../../src/memory/paging/manager/directory/read.rs"]
mod read;
#[path = "../../../../src/memory/paging/manager/directory/slot.rs"]
mod slot;
#[path = "../../../../src/memory/paging/manager/directory/table.rs"]
mod table;
#[cfg(test)]
mod tests;
#[path = "../../../../src/memory/paging/manager/directory/write.rs"]
mod write;

/// The directory is one table for the whole process, as in the kernel, so
/// the tests that change it take turns.
#[cfg(test)]
static TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());
