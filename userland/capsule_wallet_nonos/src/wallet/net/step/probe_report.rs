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

//! The probe's findings as the status the settings screen shows, and its one
//! line on the serial console.

use super::Probe;
use crate::wallet::net::probe_rpc_tcp::TcpProbe;
use crate::wallet::net::status::NetStatus;

impl Probe {
    pub(super) fn status(
        &self,
        reached: Option<(TcpProbe, crate::wallet::net::probe_tls_rpc::TlsProbe)>,
        failed: Option<&'static str>,
    ) -> NetStatus {
        let none = TcpProbe { connect: false };
        let (tcp, tls) =
            reached.unwrap_or((none, crate::wallet::net::probe_tls_rpc::TlsProbe::blocked()));
        let route_ready = self.route_ready();
        /* A network's refusal, or no route at all, is said in its own words. */
        let refused = failed;
        NetStatus {
            rpc_connect_ok: tcp.connect,
            tls_chain_ok: tls.chain,
            rpc_chain_ok: tls.chain_id,
            rpc_host: "",
            route: Some(self.route),
            status: crate::wallet::net::probe_status::probe_status(
                &tls,
                tcp.connect,
                route_ready,
                refused,
            ),
        }
    }
}

/* Said when the answer came but named another network's chain id. */
pub(super) const OTHER_CHAIN: &str = "the RPC host answered, but not with this network's chain id";

/// One line on the serial console for each probe: the route, how it ended,
/// and how many answer bytes came. Nothing of the answer itself.
pub(super) fn serial(route: &str, ended: &str, answer_len: usize) {
    let line =
        alloc::format!("[wallet] rpc probe over {route}: {ended} ({answer_len} answer bytes)\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
