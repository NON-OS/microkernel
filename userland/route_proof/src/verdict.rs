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

//! What an observer may conclude about the route, and nothing more.
//!
//! Anonymous is concluded only from a fresh report, by the transport for the
//! route the system is set to, of a route that is up, over a directory that a
//! quorum of authorities signed and that is still valid, with every hop
//! authenticated. Anything short of that is not anonymous, and the reason is
//! named. A route set to an anonymity network that is not up carries nothing
//! (the transports fail closed), so it is reported as not established, never
//! as exposed; only the direct route is exposed.

use crate::report::{Network, RouteReport, Stage};

/// The policy store's route values (policy_proto::route).
pub const ROUTE_NYM: u8 = 0;
pub const ROUTE_ANYONE: u8 = 1;
pub const ROUTE_DIRECT: u8 = 2;

/// A report older than this says nothing about now.
pub const STALE_AFTER_MS: u64 = 30_000;

/// Why a chosen anonymity route is not established.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stale {
    /// The transport has said nothing since the board started.
    NoReport,
    /// It said something, too long ago to describe now.
    Old,
    /// It is not up yet, or stopped.
    Stage(Stage),
    /// Fewer authorities signed its directory than it requires, or it requires none.
    DirectoryUnsigned,
    /// The directory it holds has run out.
    DirectoryExpired,
    /// No route is open to carry traffic.
    NoOpenRoute,
    /// A hop of the open route has not proved who it is.
    HopUnauthenticated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteVerdict {
    /// Proven from the transport's own checks, all of them fresh.
    Anonymous(Network),
    /// Set to the direct route: every site sees this machine's address.
    Exposed,
    /// Set to an anonymity network that is not carrying traffic now. Nothing
    /// leaves by another route, and nothing about anonymity is proven.
    NotEstablished(Network, Stale),
    /// The route the system is set to could not be read.
    Unknown,
}

/// The verdict for the route `chosen` (a policy value, `None` when the policy
/// store did not answer), given the latest report for that route's network and
/// its age in milliseconds.
pub fn route_verdict(chosen: Option<u8>, latest: Option<(RouteReport, u64)>) -> RouteVerdict {
    let network = match chosen {
        Some(ROUTE_NYM) => Network::Nym,
        Some(ROUTE_ANYONE) => Network::Anyone,
        Some(ROUTE_DIRECT) => return RouteVerdict::Exposed,
        _ => return RouteVerdict::Unknown,
    };
    let not = |why| RouteVerdict::NotEstablished(network, why);
    let Some((r, age_ms)) = latest else {
        return not(Stale::NoReport);
    };
    // A report about the other network says nothing about this one.
    if r.network != network {
        return not(Stale::NoReport);
    }
    if age_ms > STALE_AFTER_MS {
        return not(Stale::Old);
    }
    if r.stage != Stage::Ready {
        return not(Stale::Stage(r.stage));
    }
    if r.signatures_required == 0 || r.signatures_verified < r.signatures_required {
        return not(Stale::DirectoryUnsigned);
    }
    if r.valid_for_ms <= age_ms {
        return not(Stale::DirectoryExpired);
    }
    if r.routes_open == 0 || r.hops == 0 {
        return not(Stale::NoOpenRoute);
    }
    if r.hops_authenticated < r.hops {
        return not(Stale::HopUnauthenticated);
    }
    RouteVerdict::Anonymous(network)
}
