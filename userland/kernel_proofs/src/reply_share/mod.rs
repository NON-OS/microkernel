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
 * How many calls may wait on one service's reply, included by path.
 *
 * A timed-out call keeps its entry until the service answers, so one process
 * calling a service that never answers could take all 64 places and leave
 * every other caller with EBUSY. The tests hold one caller to its share.
 */

#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/share.rs"]
pub mod share;
mod tests;
