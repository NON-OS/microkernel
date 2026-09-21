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

//! What to do after one read of a document body.

/// The decision a single read leads to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Keep,
    Drain,
    Stop,
    Wait,
}

/*
 * Kept apart from the reads so it can be checked without a socket. The rule this
 * encodes is the one the transport got wrong: a quiet moment in the middle of a
 * transfer is not the end of it.
 *
 * The old rule counted empty reads and called four hundred of them the close. An
 * empty read costs an IPC round trip and a yield, so four hundred pass in well
 * under a millisecond, while the gap between two segments of a 370 kB consensus is
 * milliseconds. Every large document ended early, failed to inflate, and was
 * reported as a body the authority would not serve.
 */

/// What to do, given what the last read returned and what net.tcp says.
///
pub fn step(read: usize, closed: bool, expired: bool) -> Step {
    if read > 0 {
        return Step::Keep;
    }
    if closed {
        return Step::Drain;
    }
    if expired {
        return Step::Stop;
    }
    Step::Wait
}
