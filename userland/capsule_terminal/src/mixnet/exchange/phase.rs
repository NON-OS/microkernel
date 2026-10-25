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

//! Where an exchange has got to, and what it says of that.

use alloc::vec::Vec;

use nonos_route_link::RouteOpening;

/// Where an exchange has got to.
pub(super) enum Phase {
    /// Nothing done yet: the first step opens the connection.
    Start,
    /// A tunnel through the chosen network, opened a slice at a time.
    Opening(RouteOpening),
    /// Carrying `out` to the far end; then `After`.
    Send(After),
    /// TLS: reading the server's handshake flight.
    Flight,
    /// Reading the response.
    Response,
    /// Over; the result has been handed back.
    Done,
}

#[derive(Clone, Copy)]
pub(super) enum After {
    Flight,
    Response,
}

/// What a person is shown of an exchange's progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Connecting,
    Handshake,
    Sending,
    Receiving,
}

pub enum Poll {
    Pending,
    /// The response's bytes (decrypted, for TLS), or why there are none.
    Ready(Result<Vec<u8>, &'static str>),
}
