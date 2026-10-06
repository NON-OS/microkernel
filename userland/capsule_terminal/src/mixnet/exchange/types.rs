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

//! One request and its response, carried a tick at a time.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_route_link::{Route, RouteOpening, RouteStream};
use nonos_tls::flight::ClientFlight;
use nonos_tls::{AppReader, HandshakeState, TrafficKeys};

use super::wait::Wait;

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

pub(super) struct Tls {
    pub(super) client: ClientFlight,
    pub(super) handshake: Option<HandshakeState>,
    pub(super) app: Option<TrafficKeys>,
    pub(super) reader: AppReader,
}

pub struct Exchange {
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) route: Route,
    /// Whether the connection crosses an anonymity network, whose waits are
    /// longer: the route's own answer, or an .anyone service reached on any.
    pub(super) anonymous: bool,
    pub(super) secure: bool,
    /// The wall clock certificates are judged against.
    pub(super) rtc: u64,
    /// The request as HTTP; TLS seals it behind its Finished.
    pub(super) request: Vec<u8>,
    pub(super) limit: usize,
    pub(super) phase: Phase,
    pub(super) stream: Option<RouteStream>,
    /// Bytes being carried, and how many of them have gone.
    pub(super) out: Vec<u8>,
    pub(super) sent: usize,
    /// The flight, then the response, as they arrive.
    pub(super) raw: Vec<u8>,
    pub(super) tls: Option<Tls>,
    pub(super) wait: Wait,
}
