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
 * The files in a process's /proc directory.
 *
 * Every figure is the family's own: the image record for the name and the
 * argument block, the region list for the address space, and the kernel's
 * own count of the process's ticks, faults and resident pages. What the
 * personality cannot measure is left out, never made up.
 */

mod files;
mod stat;
mod statm;

pub use files::{content, usage, vsize};
