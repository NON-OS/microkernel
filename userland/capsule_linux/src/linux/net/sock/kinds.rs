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

//! What kind of socket, in which family, at which IPv4 address.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Proto {
    Stream,
    Dgram,
}

/// AF_INET, or AF_UNIX for the two ends socketpair makes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Inet,
    Unix,
}

/// An IPv4 address and a port, the port in host order.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Addr {
    pub ip: [u8; 4],
    pub port: u16,
}

/// How a stream connect to a listener went.
pub enum Link {
    /// Connected; the listener has one more connection to accept.
    Done,
    /// Nothing listens there: Linux's loopback answers with a reset.
    Refused,
    /// The listener's queue is full.
    Full,
}
