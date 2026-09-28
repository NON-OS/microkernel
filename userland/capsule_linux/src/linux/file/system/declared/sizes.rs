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

/* The sizes a guest is told: CPUs, memory, pids, ticks, pipes and pages. */

/*
 * One CPU, as sched_getaffinity says: a family never learns how many the
 * machine has.
 */
pub const CPUS: u64 = 1;

/*
 * The memory a family may address, which is RLIMIT_AS: MemTotal and
 * sysinfo's totalram are this, not the machine's memory.
 */
pub const MEMORY: u64 = 0x0000_7FFF_F000;

/* Linux's own default for a machine of 32 CPUs or fewer. */
pub const PID_MAX: u64 = 32768;

/* Clock ticks a second, as AT_CLKTCK tells the C runtime. */
pub const HZ: u64 = 100;

/*
 * A pipe holds 64 KiB (call/pipe_io.rs), and F_SETPIPE_SZ is not served,
 * so that is also the most a pipe can be given.
 */
pub const PIPE_MAX: u64 = 64 << 10;

/*
 * Anonymous memory is reserved without frames and filled on first touch,
 * so any reservation below the limit succeeds: Linux's "always" (1).
 */
pub const OVERCOMMIT: u64 = 1;

/*
 * x86_64's second-level page, which is what Go reads to size its heap
 * arenas. An architectural constant, the same on every x86_64 machine.
 */
pub const HPAGE_PMD: u64 = 2 << 20;

/*
 * The most the family may keep in its private directories, all of them
 * together: half its memory, as Linux sizes a tmpfs it is given no size.
 */
pub const PRIVATE: u64 = MEMORY / 2;
