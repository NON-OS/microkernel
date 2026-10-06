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

//! The board's words. Pure, so what the screen says is proven against what
//! was concluded (attest_doc_proofs): no headline can claim more than the
//! verdict under it.

use nonos_route_proof::{Network, RouteReport, RouteVerdict, Stage, Stale};

use super::session::Headline;

pub fn headline(h: Headline) -> &'static [u8] {
    match h {
        Headline::AttestedAndAnonymous(Network::Nym) => b"Attested, and anonymous over the Nym mixnet",
        Headline::AttestedAndAnonymous(Network::Anyone) => b"Attested, and anonymous over the Anyone network",
        Headline::AttestedNotAnonymous => b"Attested, not anonymous: the direct route shows this machine",
        Headline::AttestedRouteDown => b"Attested; the anonymity route is not up, so nothing leaves",
        Headline::NotAttested => b"Not attested: a check below failed",
        Headline::Unknown => b"Not established: something below could not be read",
    }
}

pub fn network(n: Network) -> &'static [u8] {
    match n {
        Network::Nym => b"Nym mixnet",
        Network::Anyone => b"Anyone network",
    }
}

pub fn stage(s: Stage) -> &'static [u8] {
    match s {
        Stage::Cold => b"not started",
        Stage::Bootstrapping => b"fetching the directory",
        Stage::Joining => b"directory verified, building the route",
        Stage::Ready => b"up",
        Stage::Failed => b"stopped on an error",
    }
}

/// Why the route the system is set to is not established.
pub fn why(s: Stale) -> &'static [u8] {
    match s {
        Stale::NoReport => b"the transport has not reported",
        Stale::Old => b"the last report is more than thirty seconds old",
        Stale::Stage(_) => b"the transport is not up yet",
        Stale::DirectoryUnsigned => b"too few authorities signed its directory",
        Stale::DirectoryExpired => b"its directory has expired",
        Stale::NoOpenRoute => b"no route is open",
        Stale::HopUnauthenticated => b"a hop has not proved who it is",
    }
}

/// The route verdict as the evidence column shows it.
pub fn route_evidence(v: RouteVerdict) -> &'static [u8] {
    match v {
        RouteVerdict::Anonymous(n) => network(n),
        RouteVerdict::Exposed => b"direct",
        RouteVerdict::NotEstablished(_, s) => why(s),
        RouteVerdict::Unknown => b"route not readable",
    }
}

/// `n of m`, written into `out`.
pub fn of(n: u64, m: u64, out: &mut [u8; 48]) -> &[u8] {
    let mut w = Writer { out, len: 0 };
    w.num(n);
    w.text(b" of ");
    w.num(m);
    w.done()
}

/// A duration in milliseconds as the coarsest honest unit: "45 s", "12 min",
/// "3 h 20 min", "2 d 4 h".
pub fn duration(ms: u64, out: &mut [u8; 48]) -> &[u8] {
    let s = ms / 1_000;
    let mut w = Writer { out, len: 0 };
    if s < 60 {
        w.num(s);
        w.text(b" s");
    } else if s < 3_600 {
        w.num(s / 60);
        w.text(b" min");
    } else if s < 86_400 {
        w.num(s / 3_600);
        w.text(b" h ");
        w.num((s % 3_600) / 60);
        w.text(b" min");
    } else {
        w.num(s / 86_400);
        w.text(b" d ");
        w.num((s % 86_400) / 3_600);
        w.text(b" h");
    }
    w.done()
}

/// The line under a transport's row: directory, nodes and validity.
pub fn directory<'a>(r: &RouteReport, out: &'a mut [u8; 48]) -> &'a [u8] {
    let mut w = Writer { out, len: 0 };
    w.num(u64::from(r.signatures_verified));
    w.text(b" of ");
    w.num(u64::from(r.signatures_required));
    w.text(b" signatures, ");
    w.num(u64::from(r.nodes));
    w.text(b" nodes");
    w.done()
}

/// The hop line: "3 of 3 hops authenticated".
pub fn hops<'a>(r: &RouteReport, out: &'a mut [u8; 48]) -> &'a [u8] {
    let mut w = Writer { out, len: 0 };
    w.num(u64::from(r.hops_authenticated));
    w.text(b" of ");
    w.num(u64::from(r.hops));
    w.text(b" hops authenticated");
    w.done()
}

/// Writes into a fixed buffer and stops at its end: a long line is cut, never
/// an overflow.
struct Writer<'a> {
    out: &'a mut [u8; 48],
    len: usize,
}

impl<'a> Writer<'a> {
    fn text(&mut self, b: &[u8]) {
        for &c in b {
            if self.len == self.out.len() {
                return;
            }
            self.out[self.len] = c;
            self.len += 1;
        }
    }
    fn num(&mut self, mut n: u64) {
        let mut digits = [0u8; 20];
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        let tmp = digits;
        self.text(&tmp[i..]);
    }
    fn done(self) -> &'a [u8] {
        let len = self.len;
        &self.out[..len]
    }
}
