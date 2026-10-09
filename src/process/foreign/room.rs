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
 * How many guests, threads included, one supervisor may hold at once.
 *
 * Every guest and every guest thread is a kernel process with a pid and a
 * kernel stack, and nothing counted them: a supervisor could create them
 * until the pid space or the kernel heap ran out, for every capsule on the
 * machine. The Linux personality now holds each family to 512 tasks, but the
 * kernel cannot rely on a supervisor to hold itself. One supervisor now holds
 * at most MAX_GUESTS; past that a spawn, fork or thread is EAGAIN, as Linux
 * answers a fork past RLIMIT_NPROC. A guest leaves the count in its own
 * teardown, so ended guests free their places at once. The check comes
 * before the process is made; two calls racing from one supervisor's threads
 * can pass it together, so the bound is the ceiling plus that supervisor's
 * thread count, never unbounded.
 */

pub(crate) const MAX_GUESTS: usize = 1024;

/// Whether a supervisor holding `held` guests may make one more.
pub(crate) fn has_room(held: usize) -> bool {
    held < MAX_GUESTS
}
