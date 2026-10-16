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

//! The network check a step at a time: the wallet's route (Nym, or Anyone
//! when Nym is down; never Direct), then the RPC host over it as one
//! `Exchange` that asks for the chain id once the handshake finished. It
//! opens nothing else: every probe is traffic an observer can see.

use super::super::private_route::refused;
use super::super::status::NetStatus;
use super::super::WalletRoute;
use super::exchange::Exchange;

#[path = "probe_end.rs"]
mod probe_end;
#[path = "probe_report.rs"]
mod probe_report;

pub struct Probe {
    step: u8,
    route: WalletRoute,
    exchange: Option<Exchange>,
}

impl Probe {
    /// One step; the finished diagnostic once there is one.
    pub fn step(&mut self) -> Option<NetStatus> {
        match self.step {
            0 => self.route = WalletRoute::for_wallet(),
            1 => {
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
