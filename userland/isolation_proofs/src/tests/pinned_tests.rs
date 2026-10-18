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

//! nonos_proof_abi::isolation::STATEMENT is the circuit's: the image byte for
//! byte, every number beside it, and the honest proof verifies under it.

use crate::image::{encode, pins, record};
use isolation_stark::{shape, QUERIES, STATEMENT_BLOWUP_BITS, STATEMENT_GRIND_BITS, WORDS};
use nonos_proof_abi::isolation::STATEMENT as PIN;
use nonos_stark::air::domain_params_blown;

use super::fixture::proven;

#[test]
fn the_pinned_statement_is_the_circuits() {
    let (st, proof) = proven();
    let air = shape(st).expect("the shape");
    let program = encode(&air, &record(&air));
    assert!(PIN.program == program.as_slice(), "program.bin is stale");
    let p = pins(&air, &program);
    assert_eq!(PIN.program_hash, p.program_hash);
    assert_eq!(PIN.periodic_root, p.periodic_root);
    let (a, b) = (&PIN.shape, &p.shape);
    assert_eq!(
        (a.trace_width, a.region_width, a.window, a.constraint_degree),
        (b.trace_width, b.region_width, b.window, b.constraint_degree)
    );
    assert_eq!((a.mask_pair, a.challenge_lanes), (b.mask_pair, b.challenge_lanes));
    assert_eq!((PIN.extra_blowup_bits, PIN.words), (STATEMENT_BLOWUP_BITS, WORDS));
    let point = &PIN.points[0];
    assert_eq!(PIN.points.len(), 1);
    assert_eq!(
        (point.params, point.queries, point.grind_bits),
        (p.params, QUERIES, STATEMENT_GRIND_BITS)
    );
    // L1-Shield's format-7 adapter takes log domain 23 at rate bits 6, and
    // nothing else.
    assert_eq!(domain_params_blown(&air, STATEMENT_BLOWUP_BITS), (23, 6));
    assert!(proof.len() <= PIN.max_proof_bytes, "a {} byte proof", proof.len());
    assert_eq!(nox_verify::verify_why(&PIN, proof, &st.words()), Ok(()));
}
