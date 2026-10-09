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

//! How many bytes one send takes from the guest.

/// What one send takes from the guest at most: the default receive buffer
/// of a stream's peer, so a single call can fill it.
const STREAM_CAP: usize = 128 << 10;
/// One byte past the largest datagram, so a larger one is seen and refused.
const GRAM_CAP: usize = 65508;

/// The most one send gathers: what net.sockets carries in a call, a
/// stream peer's default queue, or one byte past the largest datagram.
pub fn cap(outside: bool, stream: bool) -> usize {
    match (outside, stream) {
        (true, _) => super::stream::MAX_IO,
        (false, true) => STREAM_CAP,
        (false, false) => GRAM_CAP,
    }
}
