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

//! What a read can bring back, and what reads are made from.

/// What one read brought back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Recv {
    /// This many bytes, at the front of the caller's buffer.
    Bytes(usize),
    /// Nothing is waiting yet. A socket through net.sockets also reads this
    /// way once its peer has closed it: neither net.sockets nor net.tcp says
    /// a close apart from an empty socket.
    Empty,
    /// The reply never arrived. The read keeps its number, so asking again
    /// is answered with the same bytes rather than losing them.
    Lost,
    /// The far end finished and everything it sent has been read: nothing
    /// more will come. A proxy says so with its answer's marker, and one
    /// that can no longer be asked at all is as good as closed.
    Closed,
}

/// Something reads can be made from, with a clock to bound them by.
pub trait Source {
    fn recv(&mut self, handle: u32, out: &mut [u8]) -> Recv;
    fn now_ms(&self) -> i64;
}
