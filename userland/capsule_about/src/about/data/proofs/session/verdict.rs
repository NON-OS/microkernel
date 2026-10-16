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

//! The headline, never more than every check under it.

use super::marks::Mark;
use nonos_route_proof::{Network, RouteVerdict};

/// The headline over the whole screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Headline {
    /// Everything below holds.
    AttestedAndAnonymous(Network),
    /// Attested, and set to the direct route.
    AttestedNotAnonymous,
    /// Attested; the anonymity route is not carrying traffic yet.
    AttestedRouteDown,
    /// Something that should hold does not.
    NotAttested,
    /// Not enough could be read to say.
    Unknown,
}

pub fn headline(boot: Mark, admitted: Mark, proven: Mark, route: RouteVerdict) -> Headline {
    let attested = all([boot, admitted, proven]);
    match (attested, route) {
        (Mark::Broken, _) => Headline::NotAttested,
        (Mark::Unknown, _) => Headline::Unknown,
        (Mark::Holds, RouteVerdict::Anonymous(n)) => Headline::AttestedAndAnonymous(n),
        (Mark::Holds, RouteVerdict::Exposed) => Headline::AttestedNotAnonymous,
        (Mark::Holds, RouteVerdict::NotEstablished(..)) => Headline::AttestedRouteDown,
        (Mark::Holds, RouteVerdict::Unknown) => Headline::Unknown,
    }
}

pub(super) fn all<const N: usize>(marks: [Mark; N]) -> Mark {
    if marks.contains(&Mark::Broken) {
        Mark::Broken
    } else if marks.iter().all(|m| *m == Mark::Holds) {
        Mark::Holds
    } else {
        Mark::Unknown
    }
}
