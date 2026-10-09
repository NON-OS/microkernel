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

//! The network self-diagnostic a step at a time: what `probe_network` asks,
//! in its order. The local services are asked one per step, each the
//! bounded health call it always was, then the codec and TLS self checks,
//! then the RPC host over the chosen route as an `Exchange` that asks for
//! the chain id once the handshake finished.

use nonos_route_link::Route;

use super::super::constants::{
    DNS_MAGIC, NYM_MAGIC, SERVICE_DNS, SERVICE_NYM, SERVICE_SOCKETS, SOCKETS_MAGIC,
};
use super::super::probe_rpc_tcp::TcpProbe;
use super::super::status::NetStatus;
use super::exchange::{Ended, Exchange};

pub struct Probe {
    step: u8,
    route: Route,
    ports: [u32; 3],
    dns_ok: bool,
    sockets_ok: bool,
    nym_ok: bool,
    rpc_codec_ok: bool,
    tls13_ok: bool,
    exchange: Option<Exchange>,
}

impl Probe {
    pub fn begin() -> Probe {
        Probe {
            step: 0,
            route: Route::Down(""),
            ports: [0; 3],
            dns_ok: false,
            sockets_ok: false,
            nym_ok: false,
            rpc_codec_ok: false,
            tls13_ok: false,
            exchange: None,
        }
    }

    /// One step; the finished diagnostic once there is one.
    pub fn step(&mut self) -> Option<NetStatus> {
        let lookup = super::super::lookup::lookup;
        let health = |port: u32, magic| port != 0 && super::super::health::health(port, magic);
        match self.step {
            0 => {
                self.route = Route::for_wallet();
                self.ports = [lookup(SERVICE_DNS), lookup(SERVICE_SOCKETS), lookup(SERVICE_NYM)];
            }
            1 => self.dns_ok = health(self.ports[0], DNS_MAGIC),
            2 => self.sockets_ok = health(self.ports[1], SOCKETS_MAGIC),
            3 => self.nym_ok = health(self.ports[2], NYM_MAGIC),
            4 => self.rpc_codec_ok = crate::wallet::rpc::self_check(),
            /* The TLS client can make its key shares and build a hello:
             * the crypto service answers it. */
            5 => self.tls13_ok = nonos_tls::client_flight(b"probe.invalid").is_some(),
            6 => {
                if !self.route_ready() {
                    return Some(self.status(None, Some(self.route.name())));
                }
                let body = crate::wallet::rpc::request_chain_id(1);
                self.exchange = Some(Exchange::begin(self.route, body));
            }
            _ => {
                let ended = self.exchange.as_mut()?.step()?;
                let (chain_id, why) = match ended {
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
                let unopened = exchange.unopened;
                let mut status = self.status(Some((exchange.tcp, tls)), unopened);
                status.rpc_host = exchange.host;
                /* Once the server answered the hello, the reason the
                 * connection went no further is the status, not how far
                 * the handshake got: a refused certificate says why. */
                if let (true, false, Some(why)) = (tls.server_hello, tls.chain_id, why) {
                    status.status = why.as_bytes();
                }
                return Some(status);
            }
        }
        self.step = self.step.saturating_add(1);
        None
    }

    /* Direct needs the local stack; the others need their proxy, which the
     * route already found running, or it would be down. */
    fn route_ready(&self) -> bool {
        match self.route {
            Route::Direct => self.dns_ok && self.sockets_ok,
            Route::Nym(_) | Route::Anon(_) => true,
            Route::Down(_) => false,
        }
    }

    fn status(
        &self,
        reached: Option<(TcpProbe, super::super::probe_tls_rpc::TlsProbe)>,
        failed: Option<&'static str>,
    ) -> NetStatus {
        let none = TcpProbe { resolve: false, socket: false, connect: false };
        let (tcp, tls) =
            reached.unwrap_or((none, super::super::probe_tls_rpc::TlsProbe::blocked()));
        let route_ready = self.route_ready();
        /* A direct failure reads as it always did; an anonymous network's
         * refusal, or a route that is down, is said in its own words. */
        let refused = failed.filter(|_| self.route != Route::Direct);
        NetStatus {
            dns_ok: self.dns_ok,
            sockets_ok: self.sockets_ok,
            nym_ok: self.nym_ok,
            route_ready,
            rpc_resolve_ok: tcp.resolve,
            rpc_socket_ok: tcp.socket,
            rpc_connect_ok: tcp.connect,
            rpc_tcp_ok: tcp.connect,
            rpc_codec_ok: self.rpc_codec_ok,
            tls13_ok: self.tls13_ok,
            tls_server_ok: tls.server_hello,
            tls_record_ok: tls.encrypted_record,
            tls_certificate_ok: tls.certificate,
            tls_chain_ok: tls.chain,
            tls_anchor_ok: tls.anchor,
            tls_signature_ok: tls.signature,
            tls_hostname_ok: tls.hostname,
            tls_validity_ok: tls.validity,
            tls_finished_ok: tls.finished,
            tls_client_finished_ok: tls.client_finished,
            rpc_chain_ok: tls.chain_id,
            rpc_host: "",
            route: Some(self.route),
            status: super::super::probe_status::probe_status(
                &tls,
                tcp.connect,
                route_ready,
                refused,
            ),
        }
    }
}

/* Said when the answer came but named another network's chain id. */
const OTHER_CHAIN: &str = "the RPC host answered, but not with this network's chain id";

/// One line on the serial console for each probe: the route, how it ended,
/// and how many answer bytes came. Nothing of the answer itself.
fn serial(route: &str, ended: &str, answer_len: usize) {
    let line =
        alloc::format!("[wallet] rpc probe over {route}: {ended} ({answer_len} answer bytes)\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
