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
 * Every stack in a CPU's area must keep a guard page of its own.
 *
 * The kernel's stack slot arithmetic and the constants it reads are included
 * by path. The stacks were packed back to back, so each stack's upper guard
 * was the next stack's first page. The checks below do not build against that
 * code, which had no stack_slot_offset.
 */

pub mod constants;
pub mod manager;
mod tests;
