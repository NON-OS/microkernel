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

//! What the reserved memory range nodes ask of each SMMU: for every RMR
//! mapping whose output is an SMMU node, the stream IDs it names there and
//! each range those streams must keep mapped one to one. A mapping into
//! anything but an SMMU names no stream and is passed over.

use super::kind::Kind;
use super::node::Node;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reserved {
    /// The SMMU node, by its offset in the IORT.
    pub smmu: u32,
    /// The first stream ID and the count less one; a single mapping names
    /// one stream, its output base.
    pub first_stream: u32,
    pub streams_less_one: u32,
    pub base: u64,
    pub length: u64,
}

pub fn reserved(nodes: &[Node]) -> Vec<Reserved> {
    let smmu = |at: u32| {
        nodes.iter().find(|n| n.offset == at).is_some_and(|n| {
            matches!(n.kind, Kind::Smmuv2 { .. } | Kind::Smmuv3 { .. })
        })
    };
    let mut out = Vec::new();
    for n in nodes {
        let Kind::Rmr { ranges, .. } = &n.kind else { continue };
        for m in n.mappings.iter().filter(|m| smmu(m.output_ref)) {
            let streams_less_one = if m.single { 0 } else { m.count_less_one };
            let (smmu, first_stream) = (m.output_ref, m.output_base);
            for &(base, length) in ranges {
                out.push(Reserved { smmu, first_stream, streams_less_one, base, length });
            }
        }
    }
    out
}
