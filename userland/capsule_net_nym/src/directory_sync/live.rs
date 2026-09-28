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

use alloc::vec::Vec;

use super::roles::fetch_role;
use crate::topology::{Node, Role};

/// The endpoint the node list is asked for. One name, rather than a frozen
/// copy of what it answered when the image was built.
pub(super) const API_HOST: &str = "validator.nymtech.net";
/// The skimmed active views. Nym folded mixnodes and gateways into one node
/// type, so the older split paths answer 404. Skimmed rather than described
/// because it carries exactly what a route needs, the address, the mix port
/// and both keys, in tens of kilobytes instead of a megabyte and a half.
const MIXNODES_PATH: &str = "/api/v1/unstable/nym-nodes/skimmed/mixnodes/active";
const GATEWAYS_PATH: &str = "/api/v1/unstable/nym-nodes/skimmed/entry-gateways/active";
const EXITS_PATH: &str = "/api/v1/unstable/nym-nodes/skimmed/exit-gateways/active";

/// The mix layers, which are what a route is built from.
pub(super) fn fetch_mixnodes(tcp_port: u32) -> Result<Vec<Node>, u16> {
    fetch_role(tcp_port, MIXNODES_PATH, Role::Mix)
}

/// The entry gateways a client can hold a session with.
pub(super) fn fetch_gateways(tcp_port: u32) -> Result<Vec<Node>, u16> {
    fetch_role(tcp_port, GATEWAYS_PATH, Role::EntryGateway)
}

/// The gateways a packet leaves the mixnet by. A route ends at one of these,
/// and the exit that opens connections runs behind it.
pub(super) fn fetch_exits(tcp_port: u32) -> Result<Vec<Node>, u16> {
    fetch_role(tcp_port, EXITS_PATH, Role::ExitGateway)
}
