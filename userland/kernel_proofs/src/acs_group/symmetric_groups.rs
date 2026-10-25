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

//! Each function's group on a generated machine, as confine reads it: its
//! own path, its own ACS at every hop, members across a shared conventional
//! bus marked.

use super::acs_path::path;
use super::acs_reach::{group_by_hop, shared};
use super::acs_tree::Bdf;
use super::member::Member;
use super::symmetric_topologies::Machine;

pub(super) fn groups(m: &Machine) -> Vec<Vec<Member>> {
    let at = |b: Bdf| m.funcs.iter().position(|f| f.at == b);
    (0..m.funcs.len())
        .map(|i| {
            let Some(hops) = path(m.funcs[i].at, &m.funcs) else { return Vec::new() };
            let across = shared(&hops);
            let read = |b: Bdf, _: u16| at(b).and_then(|j| m.acs[j]);
            group_by_hop(hops, read)
                .into_iter()
                .filter_map(|(member, hop)| {
                    let id = at(member)? as u64;
                    Some(Member { id, at: member, hop, shared: across.contains(&member) })
                })
                .collect()
        })
        .collect()
}
