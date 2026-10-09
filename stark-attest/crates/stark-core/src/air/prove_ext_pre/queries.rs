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

use super::super::super::field::{Fp, Fp2};
use super::super::super::merkle::MerkleTree;
use super::super::super::transcript::Transcript;
use super::super::prove_ext::{eval_base, Domain};
use super::super::types_ext::StarkQueryExt;
use super::super::types_ext_pre::PeriodicOpeningExt;
use alloc::vec::Vec;

/// Draw the query positions and open everything at them, the periodic sidecar
/// included. Trace and periodic values are evaluated from coefficients on
/// demand; the trees hold the commitments.
#[allow(clippy::too_many_arguments)]
pub(super) fn open(
    transcript: &mut Transcript,
    n_queries: usize,
    d: &Domain,
    trace: &[Vec<Fp>],
    trace_tree: &MerkleTree,
    periodic: &[Vec<Fp>],
    periodic_tree: &MerkleTree,
    comp_d: &[Fp2],
    comp_tree: &MerkleTree,
    deep_d: &[Fp2],
    deep_tree: &MerkleTree,
) -> (Vec<StarkQueryExt>, Vec<PeriodicOpeningExt>) {
    let mut queries = Vec::with_capacity(n_queries);
    let mut openings = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(d.n);
        let x_p = d.shift * d.omega.pow(p as u64);
        queries.push(StarkQueryExt {
            deep: deep_d[p],
            deep_path: deep_tree.open(p),
            trace: trace.iter().map(|cf| eval_base(cf, x_p)).collect(),
            trace_path: trace_tree.open(p),
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
        });
        openings.push(PeriodicOpeningExt {
            row: periodic.iter().map(|cf| eval_base(cf, x_p)).collect(),
            path: periodic_tree.open(p),
        });
    }
    (queries, openings)
}
