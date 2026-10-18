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

//! What a UART transport needs from the serial port a capsule drives:
//! octets out, the octets that have come in, and time. Reads never wait,
//! so every wait above this is bounded by the caller's own polls.

pub trait UartPort {
    /// Send every octet of `data`, in order, or the errno that stopped it.
    fn write(&mut self, data: &[u8]) -> Result<(), i32>;
    /// Copy into `out` the octets received since the last read, at most
    /// `out.len()`; 0 when none have come.
    fn read(&mut self, out: &mut [u8]) -> Result<usize, i32>;
    /// Milliseconds on a clock that never goes back, for the Three-wire
    /// UART's retransmission and link establishment timers.
    fn now_ms(&mut self) -> u64;
    /// Let about a millisecond pass between two empty reads.
    fn pause(&mut self);
}
