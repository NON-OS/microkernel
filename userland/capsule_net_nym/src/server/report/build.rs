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

//! The facts this transport checked itself, for the board's report.

use nonos_route_proof::{nym_report, NymDirectory, NymFacts, RouteReport};

use crate::state::TABLE;
use crate::topology::{self, TopologyStatus, ROUTE_HOPS};

/// How long after a cover tick fell due the transport still counts as
/// sending cover traffic. The tick's own jitter is at most ten seconds.
const COVER_WINDOW_MS: u64 = 10_000;

/// The report at `now_ms` (the wall clock the topology's window is kept on).
///
/// No gateway, mix node or key is named: the board is readable by any local
/// caller. The gateway counts as authenticated only when the topology gave it
/// an identity and the handshake derived a key against it.
pub fn build(now_ms: u64) -> RouteReport {
    let directory = match topology::status() {
        TopologyStatus::Missing => NymDirectory::Missing,
        TopologyStatus::Clock => NymDirectory::NoClock,
        TopologyStatus::Expired => NymDirectory::Expired,
        TopologyStatus::UntrustedAuthority => NymDirectory::Untrusted,
        TopologyStatus::Ready => NymDirectory::Ready,
    };
    let valid_for_ms = topology::meta().map_or(0, |m| m.not_after_ms.saturating_sub(now_ms));
    let mut table = TABLE.lock();
    let gateway_authenticated = super::super::connect_tick::connected()
        && table.gateway().is_some_and(|g| g.identity != [0u8; 32] && g.shared_key != [0u8; 32]);
    let surbs = table.with_sphinx_session(|s| s.surbs.held()).unwrap_or(0);
    drop(table);
    let next_cover = crate::state::next_cover_ms();
    nym_report(&NymFacts {
        directory,
        nodes: topology::node_count(),
        valid_for_ms,
        gateway_authenticated,
        route_hops: ROUTE_HOPS,
        surbs,
        cover_traffic: next_cover != 0 && now_ms <= next_cover.saturating_add(COVER_WINDOW_MS),
    })
}
