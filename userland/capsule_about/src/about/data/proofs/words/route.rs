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

//! The route's evidence, and why a route is not established.

use super::verdict::network;
use nonos_route_proof::{RouteVerdict, Stale};

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
