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
 * A thread whose start the kernel would refuse is refused before it is built,
 * and one that is built and then cannot start is ended.
 *
 * The kernel's thread start rule is included by path. MkThreadSpawn and
 * MkForeignThread built a thread and published it in the process table, and
 * only then asked the user entry builder whether it could start there. A
 * refusal returned past the thread, which stayed New in the table for good
 * with its pid and its kernel stack; a capsule holding IPC could repeat that
 * until the pid space or the memory ran out. The checks below pin the rule to
 * the builder's own bound, and the spawn path to asking it first and to
 * ending a thread it published and could not start.
 */

#[path = "../../../../src/process/core/table/thread_start.rs"]
pub mod thread_start;
mod tests;
