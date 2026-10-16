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

//! How a probe starts, whether its route may carry it, and its last step:
//! the RPC host's answer judged, and the status made.

use super::super::exchange::Ended;
use super::probe_report::{serial, OTHER_CHAIN};
use super::Probe;
use crate::wallet::net::private_route::refused;
use crate::wallet::net::status::NetStatus;
use nonos_route_link::Route;

impl Probe {
    pub fn begin() -> Probe {
        let route = Route::Down("");
        Probe { step: 0, route, exchange: None }
    }

    /* Only Nym or Anyone carries the wallet: their proxy was found running,
     * or the route would be down. */
    pub(super) fn route_ready(&self) -> bool {
        refused(self.route).is_none()
    }

    pub(super) fn ended(&mut self) -> Option<NetStatus> {
        let (chain_id, why) = match self.exchange.as_mut()?.step()? {
            Ended::Answer(plain) => {
                let want = crate::wallet::chain::chain_id_answer();
                let ok = plain.windows(want.len()).any(|w| w == want.as_bytes());
                (ok, (!ok).then_some(OTHER_CHAIN))
            }
            Ended::Failed(why) | Ended::Unknown(why) => (false, Some(why)),
        };
        let exchange = self.exchange.take()?;
        let mut tls = exchange.tls;
        tls.chain_id = chain_id && tls.client_finished;
        serial(self.route.name(), why.unwrap_or("connected"), exchange.answer_len);
        let mut status = self.status(Some((exchange.tcp, tls)), exchange.unopened);
        status.rpc_host = exchange.host;
        /* Once the server answered the hello, the reason the connection went
         * no further is the status: a refused certificate says why. */
        if let (true, false, Some(why)) = (tls.server_hello, tls.chain_id, why) {
            status.status = why.as_bytes();
        }
        Some(status)
    }
}
