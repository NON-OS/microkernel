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
 * How many calls may wait on one service's reply, in all and from one caller.
 *
 * A call's entry stays queued until the service answers it, even after the
 * caller's wait timed out: a late answer has to find its own entry, whose
 * token the caller then discards, rather than be paired with the caller's
 * next call. A service that never answers a request (a one-way op called
 * with mk_ipc_call, or a request it is slow on) therefore keeps the entry
 * until the caller exits, and 64 such calls from one process left every
 * other caller of that service with EBUSY. One caller now holds at most
 * MAX_PER_CALLER of the MAX_PER_SERVICE places; past that its own calls to
 * that service are refused and nobody else's are.
 */

pub(crate) const MAX_PER_SERVICE: usize = 64;
pub(crate) const MAX_PER_CALLER: usize = 8;

/// Whether a call may join a service's queue holding `queued` entries,
/// `mine` of them this caller's.
pub(crate) fn admits(queued: usize, mine: usize) -> bool {
    queued < MAX_PER_SERVICE && mine < MAX_PER_CALLER
}
