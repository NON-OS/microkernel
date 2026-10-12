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

//! The network self-diagnostic a step at a time: the Nym service's health,
//! the codec and TLS self checks, then the RPC host over the wallet's route
//! (Nym, or Anyone when Nym is down; never Direct) as an `Exchange` that asks
//! for the chain id once the handshake finished.

use nonos_route_link::Route;

use super::super::constants::{NYM_MAGIC, SERVICE_NYM};
use super::super::private_route::refused;
use super::super::status::NetStatus;
use super::super::{health::health, lookup::lookup};
use super::exchange::Exchange;

#[path = "probe_end.rs"]
mod probe_end;
#[path = "probe_report.rs"]
mod probe_report;

pub struct Probe {
    step: u8,
    route: Route,
    nym_port: u32,
    nym_ok: bool,
    rpc_codec_ok: bool,
    tls13_ok: bool,
    exchange: Option<Exchange>,
}

impl Probe {
    /// One step; the finished diagnostic once there is one.
    pub fn step(&mut self) -> Option<NetStatus> {
        match self.step {
            0 => (self.route, self.nym_port) = (Route::for_wallet(), lookup(SERVICE_NYM)),
            1 => self.nym_ok = self.nym_port != 0 && health(self.nym_port, NYM_MAGIC),
            2 => self.rpc_codec_ok = crate::wallet::rpc::self_check(),
            /* The TLS client can make its key shares and build a hello. */
            3 => self.tls13_ok = nonos_tls::client_flight(b"probe.invalid").is_some(),
            4 => {
                if let Some(why) = refused(self.route) {
                    return Some(self.status(None, Some(why)));
                }
                let body = crate::wallet::rpc::request_chain_id(1);
                self.exchange = Some(Exchange::begin(self.route, body));
            }
            _ => return self.ended(),
        }
        self.step = self.step.saturating_add(1);
        None
    }
}
