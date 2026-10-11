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

//! A request's exchange as it begins: the host fixed, the link to open.

use alloc::vec::Vec;

use nonos_route_link::Route;

use super::super::super::probe_rpc_tcp::TcpProbe;
use super::super::super::probe_tls_rpc::TlsProbe;
use super::super::open::Opening;
use super::state::Exchange;
use super::types::Stage;

impl Exchange {
    pub fn begin(route: Route, body: Vec<u8>) -> Exchange {
        let host = crate::wallet::chain::rpc_host();
        Exchange {
            host,
            route,
            body,
            stage: Stage::Open(Opening::begin(route, host)),
            link: None,
            flight: None,
            handshake: None,
            app: None,
            server: Vec::new(),
            flight_end: 0,
            out: Vec::new(),
            sent: 0,
            gather: None,
            now: None,
            handshake_end: None,
            tcp: TcpProbe { resolve: false, socket: false, connect: false },
            tls: TlsProbe::blocked(),
            unopened: None,
            answer_len: 0,
        }
    }
}
