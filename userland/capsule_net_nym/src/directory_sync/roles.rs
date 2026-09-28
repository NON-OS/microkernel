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

//! Fetching one role's node list and judging whether it can make a route.

use alloc::vec::Vec;

use super::api::{node_objects, parse_node};
use super::https::fetch_tls;
use super::live::API_HOST;
use crate::topology::{self, Node, Role};

/// Refuse a list that cannot make a route: three mix layers, and something to
/// enter through. A short answer is a broken answer, and installing it would
/// replace a working table with one that cannot route.
const MIN_PER_LAYER: usize = 1;

/// A node list runs to tens of kilobytes (the skimmed views measured 21,658
/// to 75,446 bytes on 2026-09-28). This bounds what one answer may allocate.
const MAX_LIST: usize = 512 * 1024;

pub(super) fn fetch_role(tcp_port: u32, path: &str, role: Role) -> Result<Vec<Node>, u16> {
    let body = fetch_tls(tcp_port, API_HOST, path, MAX_LIST)?;
    let found = node_objects(&body, topology::NODE_CAP);
    let nodes: Vec<Node> = found.iter().filter_map(|o| parse_node(o, role)).collect();
    if nodes.is_empty() {
        return Err(20);
    }
    Ok(nodes)
}

pub(super) fn layers_present(nodes: &[Node]) -> bool {
    (1u8..=3).all(|layer| {
        nodes.iter().filter(|n| n.role == Role::Mix && n.layer == layer).count() >= MIN_PER_LAYER
    })
}
