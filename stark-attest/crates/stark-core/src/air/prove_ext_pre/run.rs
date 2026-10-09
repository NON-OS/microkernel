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

use super::super::super::field::Fp2;
use super::super::super::fri_ext::fri_prove_ext;
use super::super::super::merkle::MerkleTree;
use super::super::super::transcript::Transcript;
use super::super::composition::num_coeffs;
use super::super::periodic_root::periodic_tree;
use super::super::prove_ext::{
    comp_at_z, draw_ood_point_ext, ood_frame, over_domain, periodic_at_z, trace_coeffs,
    wide_streamed, Domain,
};
use super::super::spec::AirExt;
use super::super::types_ext::StarkProofExt;
use super::super::types_ext_pre::StarkProofExtPre;
use super::{deep, queries};
use crate::field::Fp;
use alloc::vec::Vec;

/// Prove `trace` against `air` with the periodic sidecar, at the given FRI
/// rate. The verifier must hold the matching baked periodic root.
///
/// The transcript order below is the protocol and matches the materialized
/// prover this replaced; the periodic tree comes through the same helper a
/// registered root does, so the two are one object by construction. Nothing is
/// held over the full domain but the two Fp2 codewords and the leaf digests.
pub fn stark_prove_ext_preprocessed<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> StarkProofExtPre {
    let d = Domain::of(air, extra_blowup_bits);

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    let tc = trace_coeffs(trace, &d);
    let trace_tree = wide_streamed(&tc, &d);
    let trace_root = trace_tree.root();
    transcript.absorb_digest(&trace_root);

    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp2()).collect();

    let periodic_cols = air.periodic_columns();
    let (pc, p_tree) = periodic_tree(air, extra_blowup_bits);
    let comp_d = over_domain(air, &d, &tc, &pc, &coeffs);
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_ext(&mut transcript, d.shift, d.n, d.t);
    let frame = ood_frame(&tc, &d, z);
    for value in &frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let periodic_z = periodic_at_z(&d, &periodic_cols, z);
    for value in &periodic_z {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let comp_z = comp_at_z(air, &d, &frame, &periodic_z, z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..d.width * d.window + 1 + periodic_cols.len())
        .map(|_| transcript.challenge_fp2())
        .collect();
    let deep_d =
        deep::over_domain(&d, &tc, &pc, &comp_d, &frame, &periodic_z, comp_z, z, &deep_coeffs);

    let fri = fri_prove_ext(&deep_d, d.shift, d.fri_log_blowup, n_queries, grind_bits);
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let (qs, openings) = queries::open(
        &mut transcript,
        n_queries,
        &d,
        &tc,
        &trace_tree,
        &pc,
        &p_tree,
        &comp_d,
        &comp_tree,
        &deep_d,
        &deep_tree,
    );
    StarkProofExtPre {
        proof: StarkProofExt {
            trace_root,
            comp_root: comp_tree.root(),
            ood_frame: frame,
            fri,
            queries: qs,
        },
        periodic_z,
        openings,
    }
}
