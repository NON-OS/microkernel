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

//! One JSON-RPC request to the RPC host, over its own TLS 1.3 connection,
//! a step at a time: what the blocking `fetch_rpc` did in one call, cut
//! where it used to wait. Open the link, send the hello, gather the server's
//! flight, judge it, send the request, gather the answer. No step waits on
//! the network longer than a slice, so the window that steps it keeps
//! painting and taking input however slow the route is. The host is the one
//! the network picked when the request began, so a switch halfway cannot
//! send it elsewhere.
//!
//! The TLS is nonos_tls, the client every other capsule uses: one server
//! Certificate, read in RFC 8446's order, walked to a root of the trust
//! store for this host, the leaf's key signing this handshake, and the
//! server's Finished, all checked before the request is sealed.

use alloc::vec::Vec;

use nonos_tls::flight::ClientFlight;
use nonos_tls::{HandshakeState, TrafficKeys};

use super::super::super::link::Link;
use super::super::super::probe_rpc_tcp::TcpProbe;
use super::super::super::probe_tls_rpc::TlsProbe;
use super::super::super::WalletRoute;
use super::super::gather::Gather;
use super::types::Stage;

pub struct Exchange {
    /// The RPC host asked, fixed when the request began.
    pub host: &'static str,
    pub(super) route: WalletRoute,
    pub(super) body: Vec<u8>,
    pub(super) stage: Stage,
    pub(super) link: Option<Link>,
    pub(super) flight: Option<ClientFlight>,
    /// The server's flight, keyed once from its ServerHello.
    pub(super) handshake: Option<HandshakeState>,
    /// The application keys, once the flight verified.
    pub(super) app: Option<TrafficKeys>,
    pub(super) server: Vec<u8>,
    /// Where the server's flight ends in `server`; what follows is answer.
    pub(super) flight_end: usize,
    pub(super) out: Vec<u8>,
    pub(super) sent: usize,
    pub(super) gather: Option<Gather>,
    pub(super) now: Option<u64>,
    /// When the handshake must be over: one end from the hello to the
    /// checked flight, however slowly the host trickles.
    pub(super) handshake_end: Option<u64>,
    /// How far the connection got, for the probe.
    pub tcp: TcpProbe,
    /// How far the handshake got, for the probe.
    pub tls: TlsProbe,
    /// Why the connection was never made, when it was not.
    pub unopened: Option<&'static str>,
    /// How many bytes of the answer came, for the serial line.
    pub answer_len: usize,
}
