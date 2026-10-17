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

//! What a serve loop does after a receive. Pure, so it is proven on the
//! host.

/// What a serve loop does with its receive's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecvTurn {
    /// A message arrived: serve it.
    Serve,
    /// The receive waited and came back with nothing: go round again.
    Again,
    /// The receive failed without waiting, and will fail the same way at
    /// once next time (an inbox that is gone, a bad buffer): sleep before
    /// going round, so the loop holds no core.
    Park,
}

/// The kernel's answer to a receive whose timeout ran out with nothing.
pub const ERRNO_TIMEDOUT: i64 = -110;
/// How long a serve loop sleeps after a receive that failed at once.
pub const RECV_PARK_MS: u64 = 100;

/// The turn after a receive that returned `rc`.
pub const fn recv_turn(rc: i64) -> RecvTurn {
    if rc > 0 {
        RecvTurn::Serve
    } else if rc == 0 || rc == ERRNO_TIMEDOUT {
        RecvTurn::Again
    } else {
        RecvTurn::Park
    }
}
