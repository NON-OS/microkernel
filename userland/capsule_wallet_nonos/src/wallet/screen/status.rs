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

//! The status line, from what the wallet knows at this moment and nothing
//! it would like to be true. It replaced a line that said "keys sealed, TLS
//! secured, security STRONG" whatever the state.

use alloc::vec::Vec;

use crate::wallet::net::route_part;
use crate::wallet::state::State;

pub fn parts(state: &State) -> Vec<&'static str> {
    let mut out = Vec::new();
    out.push(crate::wallet::chain::current().name);
    out.push(match (state.net.tls_chain_ok, state.net.rpc_connect_ok) {
        (true, _) => "TLS 1.3",
        (false, true) => "connecting",
        (false, false) => "offline",
    });
    /* The network the requests went over, not whether net.nym is up. */
    if let Some(route) = route_part(state.net.route) {
        out.push(route);
    }
    /* "reading" only while a refresh runs: one that ended without the
     * balance says so, rather than reading for ever. */
    let wallet = state.address_ready;
    let refreshing = state.net_job.is_some();
    out.extend(super::status_words::reading(wallet, refreshing, state.balance_ready));
    out.extend(super::status_words::kept(wallet, state.vault_saved));
    out
}
