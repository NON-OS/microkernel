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

use nonos_route_link::Route;

use super::super::bounds::Bounds;
use super::super::link::Link;
use super::super::probe_rpc_tcp::TcpProbe;
use super::super::probe_tls_rpc::TlsProbe;
use super::super::read_tls_flight::{routed, ANSWER_MAX, FLIGHT_MAX};
use super::gather::{Gather, Gathered};
use super::open::Opening;
use nonos_tls::flight::ClientFlight;
use nonos_tls::{AppReader, CertProblem, HandshakeState, Progress, Refusal, Start, TrafficKeys};

const NO_ANSWER: &str = "the RPC host did not answer";
const NO_HELLO: &str = "the TLS hello could not be built";
const NOT_TRUSTED: &str = "the RPC host did not prove itself";
const NO_CHAIN: &str = "the RPC host's TLS handshake did not finish";
const HTTP_REFUSED: &str = "the RPC host answered with an HTTP error";
const CUT_SHORT: &str = "the RPC host's answer was cut short";
const NO_REPLY: &str = "the RPC host sent nothing back after the request";
const CUT_RECORD: &str = "the RPC host's answer stopped partway through a record";
const RECORD_FAILED: &str = "the RPC host's answer did not decrypt";
const ALERTED: &str = "the RPC host ended the TLS handshake with an alert";
const RETRY: &str = "the RPC host asked for a key exchange this wallet does not offer";
const UNUSABLE: &str = "the RPC host's TLS hello could not be used";
const BROKEN: &str = "the RPC host's TLS handshake did not decrypt";
const NO_CLOCK: &str = "this machine's clock could not be read, so the RPC host's \
     certificate dates cannot be checked";
const NAME_MISMATCH: &str = "the RPC host's certificate names another host";
const EXPIRED: &str = "the RPC host's certificate has expired";
const NOT_YET: &str = "the RPC host's certificate is not valid yet, or this machine's clock \
     is behind";
const UNKNOWN_ISSUER: &str = "the RPC host's certificate is from an authority this wallet \
     does not trust";
const UNREADABLE: &str = "the RPC host's certificate could not be read";

/* A direct socket says nothing of a close, so its quiet window ends it. */
const DIRECT: Bounds =
    Bounds { first_ms: 10_000, quiet_ms: 600, total_ms: 30_000, max: FLIGHT_MAX };

enum Stage {
    Open(Opening),
    Hello,
    Send { next: Next },
    Flight,
    Check,
    Answer,
}

#[derive(Clone, Copy)]
enum Next {
    Flight,
    Answer,
}

/// How a request ended.
pub enum Ended {
    /// The plaintext of the host's answer.
    Answer(Vec<u8>),
    /// Nothing of the request went: the reason, in the route's own words
    /// where it was the route.
    Failed(&'static str),
    /// The request had begun to go when the exchange broke, so the host may
    /// have acted on it, and why it broke. A broadcast that ends here must
    /// not be sent again.
    Unknown(&'static str),
}

pub struct Exchange {
    /// The RPC host asked, fixed when the request began.
    pub host: &'static str,
    route: Route,
    body: Vec<u8>,
    stage: Stage,
    link: Option<Link>,
    flight: Option<ClientFlight>,
    /// The server's flight, keyed once from its ServerHello.
    handshake: Option<HandshakeState>,
    /// The application keys, once the flight verified.
    app: Option<TrafficKeys>,
    server: Vec<u8>,
    /// Where the server's flight ends in `server`; what follows is answer.
    flight_end: usize,
    out: Vec<u8>,
    sent: usize,
    gather: Option<Gather>,
    now: Option<u64>,
    /// How far the connection got, for the probe.
    pub tcp: TcpProbe,
    /// How far the handshake got, for the probe.
    pub tls: TlsProbe,
    /// Why the connection was never made, when it was not.
    pub unopened: Option<&'static str>,
    /// How many bytes of the answer came, for the serial line.
    pub answer_len: usize,
}

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
            tcp: TcpProbe { resolve: false, socket: false, connect: false },
            tls: TlsProbe::blocked(),
            unopened: None,
            answer_len: 0,
        }
    }

    /// One step; Some once the request has ended.
    pub fn step(&mut self) -> Option<Ended> {
        let ended = match self.advance() {
            Ok(None) => return None,
            Ok(Some(plain)) => return Some(Ended::Answer(plain)),
            Err(why) if self.request_out() => Ended::Unknown(why),
            Err(why) => Ended::Failed(why),
        };
        /* A host that did not answer, refused, or did not prove itself is
         * left for the next on its network. A route that is down says
         * nothing of the host. */
        if !matches!(self.route, Route::Down(_)) {
            crate::wallet::chain::rpc_failed(self.host);
        }
        Some(ended)
    }

    /// Whether any of the request may have left: from the first write of
    /// it on. Before that only the handshake went, which asks nothing.
    fn request_out(&self) -> bool {
        matches!(self.stage, Stage::Send { next: Next::Answer } | Stage::Answer)
    }

    fn advance(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        match &mut self.stage {
            Stage::Open(opening) => match opening.step() {
                Ok(None) => Ok(None),
                Ok(Some(link)) => {
                    self.tcp = TcpProbe { resolve: true, socket: true, connect: true };
                    self.link = Some(link);
                    self.stage = Stage::Hello;
                    Ok(None)
                }
                Err(reach) => {
                    self.tcp =
                        TcpProbe { resolve: reach.resolve, socket: reach.socket, connect: false };
                    self.unopened = Some(reach.why);
                    Err(reach.why)
                }
            },
            Stage::Hello => {
                let flight = nonos_tls::client_flight(self.host.as_bytes()).ok_or(NO_HELLO)?;
                self.out = flight.record.clone();
                self.flight = Some(flight);
                self.sent = 0;
                self.stage = Stage::Send { next: Next::Flight };
                Ok(None)
            }
            Stage::Send { next } => {
                let next = *next;
                let link = self.link.as_mut().ok_or(NO_ANSWER)?;
                self.sent += link.write_slice(&self.out[self.sent..]).map_err(|_| NO_ANSWER)?;
                if self.sent >= self.out.len() {
                    let mut bounds = self.bounds();
                    if matches!(next, Next::Answer) {
                        bounds.max = ANSWER_MAX;
                    }
                    self.gather = Some(Gather::new(bounds, nonos_libc::mk_uptime_ms()));
                    self.stage = match next {
                        Next::Flight => Stage::Flight,
                        Next::Answer => Stage::Answer,
                    };
                }
                Ok(None)
            }
            Stage::Flight => {
                /* Ready once a whole Finished came or the flight cannot
                 * finish, however many records the server packed it into.
                 * Each record is opened once across the slices. */
                let flight = self.flight.take().ok_or(NO_ANSWER)?;
                let mut handshake = self.handshake.take();
                let done = |bytes: &[u8]| judged(&flight, &mut handshake, bytes);
                let gathered = self.gather_slice(done);
                self.flight = Some(flight);
                self.handshake = handshake;
                match gathered {
                    Gathered::More => Ok(None),
                    Gathered::Nothing => Err(NO_ANSWER),
                    Gathered::Whole(server) => {
                        self.server = server;
                        self.now = super::super::rtc_stamp::rtc_stamp();
                        self.stage = Stage::Check;
                        Ok(None)
                    }
                }
            }
            Stage::Check => {
                self.check()?;
                Ok(None)
            }
            Stage::Answer => match self.gather_slice(|_| false) {
                Gathered::More => Ok(None),
                Gathered::Nothing => Err(NO_REPLY),
                Gathered::Whole(answer) => {
                    self.answer_len = answer.len();
                    let app = self.app.as_ref().ok_or(NO_ANSWER)?;
                    /* Records that followed the server's Finished in its
                     * flight are numbered before the answer's. */
                    let mut wire = self.server.get(self.flight_end..).unwrap_or(&[]).to_vec();
                    wire.extend_from_slice(&answer);
                    let mut reader = AppReader::new();
                    reader.feed(app, &wire);
                    /* Why the answer is not one: a record that fails its tag,
                     * or one cut partway. */
                    if reader.is_broken() {
                        return Err(RECORD_FAILED);
                    }
                    if !crate::wallet::tls13::records_whole(&wire) {
                        return Err(CUT_RECORD);
                    }
                    let plain = reader.into_plaintext();
                    /* Only a whole 200 answer is one: an error status or a
                     * body cut short is said, never read as values. */
                    match crate::wallet::rpc::http_answer(&plain) {
                        crate::wallet::rpc::Http::Body(body) => Ok(Some(body)),
                        crate::wallet::rpc::Http::Status(_) => Err(HTTP_REFUSED),
                        crate::wallet::rpc::Http::Incomplete => Err(CUT_SHORT),
                    }
                }
            },
        }
    }

    /// Judge the server's flight in one pass, and seal the request behind
    /// the client's Finished only when every check held.
    fn check(&mut self) -> Result<(), &'static str> {
        let client = self.flight.as_ref().ok_or(NO_ANSWER)?;
        let host = self.host.as_bytes();
        if self.handshake.is_none() {
            match HandshakeState::begin(client, &self.server) {
                Start::Ready(state) => self.handshake = Some(*state),
                Start::Waiting => return Err(NO_CHAIN),
                Start::Alert(_) => return Err(ALERTED),
                Start::Retry => return Err(RETRY),
                Start::Unusable => return Err(UNUSABLE),
            }
        }
        self.tls.server_hello = true;
        let state = self.handshake.as_mut().ok_or(NO_ANSWER)?;
        let end = match state.advance(&self.server) {
            Progress::Complete(end) => end,
            Progress::Incomplete => return Err(NO_CHAIN),
            Progress::Alert(_) => return Err(ALERTED),
            Progress::Broken => return Err(BROKEN),
        };
        self.tls.encrypted_record = true;
        self.tls.certificate = true;
        let now = self.now.ok_or(NO_CLOCK)?;
        let http = crate::wallet::rpc::http_post(host, &self.body);
        let answer = match state.answer(host, now, &http) {
            Ok(answer) => answer,
            Err(Refusal::Unverified) => {
                let problem = state.cert_problem(host, now);
                self.tls.mark_refused(problem);
                return Err(refused_why(problem));
            }
            Err(Refusal::Alert(_)) => return Err(ALERTED),
            Err(Refusal::Incomplete) => return Err(NO_CHAIN),
            Err(Refusal::Seal) => return Err(NO_ANSWER),
        };
        self.tls.mark_trusted();
        /* The handshake is over: its state and the private keys go. */
        self.handshake = None;
        if let Some(client) = self.flight.as_mut() {
            client.private = [0; 32];
            client.p256_private = [0; 32];
        }
        self.app = Some(answer.app);
        self.flight_end = end;
        self.out = answer.flight;
        self.sent = 0;
        self.stage = Stage::Send { next: Next::Answer };
        Ok(())
    }

    /// One slice read into the gather under way.
    fn gather_slice(&mut self, done: impl FnMut(&[u8]) -> bool) -> Gathered {
        let (Some(link), Some(gather)) = (self.link.as_mut(), self.gather.as_mut()) else {
            return Gathered::Nothing;
        };
        let mut chunk = [0u8; 4096];
        match link.read_slice(&mut chunk) {
            Ok(read) => gather.take(&chunk[..read.n], read.ended, nonos_libc::mk_uptime_ms(), done),
            Err(()) => gather.broke(),
        }
    }

    fn bounds(&self) -> Bounds {
        match &self.link {
            Some(Link::Routed { patience_ms, .. }) => routed(*patience_ms),
            _ => DIRECT,
        }
    }
}

/* Whether the flight so far can be judged: a whole server Finished came,
 * or the server alerted, or a record failed. Keyed from the ServerHello on
 * the first look that has one whole, and kept. */
fn judged(client: &ClientFlight, handshake: &mut Option<HandshakeState>, bytes: &[u8]) -> bool {
    if handshake.is_none() {
        match HandshakeState::begin(client, bytes) {
            Start::Waiting => return false,
            Start::Ready(state) => *handshake = Some(*state),
            Start::Alert(_) | Start::Retry | Start::Unusable => return true,
        }
    }
    handshake.as_mut().is_none_or(|state| state.advance(bytes) != Progress::Incomplete)
}

/* What a person can act on, from the first thing wrong with the chain. */
fn refused_why(problem: Option<CertProblem>) -> &'static str {
    match problem {
        Some(CertProblem::NameMismatch) => NAME_MISMATCH,
        Some(CertProblem::Expired) => EXPIRED,
        Some(CertProblem::NotYetValid) => NOT_YET,
        Some(CertProblem::UnknownIssuer) => UNKNOWN_ISSUER,
        Some(CertProblem::Unreadable) => UNREADABLE,
        None => NOT_TRUSTED,
    }
}
