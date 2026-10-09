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

//! The second list: the gateways a session is held with.

use alloc::vec::Vec;

use super::budget_roles::ENTRY_BUDGET;
use super::keep::keep;
use super::live::fetch_gateways;
use super::step::{Step, PARTIAL};
use crate::topology::Node;

/// How many times to re-ask for the gateway list before moving on without it.
///
/// The gateway fetch loses its certificate hash to a busy crypto pool early in
/// boot exactly as the exit fetch does, and comes back as an empty list. A
/// directory with no gateway in it is worse than one with no exit: a route home
/// ends at the gateway holding our session, so without a gateway the directory
/// describes, no reply block can be built and every send is refused. The
/// original single fetch left that state on any transient miss. Retry it the
/// same way the exit fetch is retried.
const GATEWAY_ATTEMPTS: u32 = 12;

/// The gateways a session is held with.
///
/// Ask a few times before proceeding: an empty result here is a transient lost
/// hash, not an empty network, and installing a directory with no gateway makes
/// a route home impossible.
pub(super) fn gateways(tcp_port: u32, mut nodes: Vec<Node>) -> Step {
    for _ in 0..GATEWAY_ATTEMPTS {
        match fetch_gateways(tcp_port) {
            Ok(found) if !found.is_empty() => {
                let mut found = keep(found, ENTRY_BUDGET, b"entry gateways");
                nodes.append(&mut found);
                break;
            }
            _ => continue,
        }
    }
    *PARTIAL.lock() = Some((nodes, 2));
    Step::Progressed
}
