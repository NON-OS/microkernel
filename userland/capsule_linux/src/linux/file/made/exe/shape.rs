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

/* What a process runs, as /proc/<pid>/exe, comm, cmdline and environ read it. */

use alloc::vec::Vec;

#[derive(Clone, Default)]
pub struct Exe {
    /* The file, as the guest names it: /proc/<pid>/exe. */
    pub path: Vec<u8>,
    /*
     * The last component of the name it was started by, cut to fifteen
     * bytes as Linux cuts it: /proc/<pid>/comm.
     */
    pub comm: Vec<u8>,
    /*
     * Where argv's strings begin and the environment's end: the two runs
     * are contiguous on the stack, as on Linux, split at `env`.
     */
    pub args: u64,
    pub env: u64,
    pub end: u64,
    /*
     * The stack pointer it started with, where argc is: Linux's
     * start_stack.
     */
    pub stack: u64,
    /* Family milliseconds when it started. */
    pub start_ms: u64,
}
