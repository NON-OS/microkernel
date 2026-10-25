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

//! A nox_verify Statement over the image: the numbers the image does not
//! carry, read from the circuit, and the pins a gate holds.

use isolation_stark::{QUERIES, STATEMENT_BLOWUP_BITS, STATEMENT_GRIND_BITS, WORDS};
use nonos_stark::air::{periodic_root, Air, AirExt, Permuted, WiredMultiGen};
use nonos_stark::hash::keccak256;
use nox_verify::{Point, Shape, Statement};
use stark_proofs::proof_wire::ParamSet;

pub struct Pins {
    pub program_hash: [u8; 32],
    pub periodic_root: [u8; 32],
    pub params: [u8; 32],
    pub shape: Shape,
}

pub fn pins(air: &WiredMultiGen, program: &[u8]) -> Pins {
    let shape = Shape {
        trace_width: air.trace_width(),
        region_width: Permuted::region_width(air),
        window: air.window_size(),
        constraint_degree: air.constraint_degree(),
        mask_pair: AirExt::mask_pair(air),
        challenge_lanes: AirExt::challenge_lanes(air),
    };
    Pins {
        program_hash: keccak256(program),
        periodic_root: periodic_root(air, STATEMENT_BLOWUP_BITS),
        params: ParamSet::of(air, QUERIES, STATEMENT_GRIND_BITS, STATEMENT_BLOWUP_BITS).id(),
        shape,
    }
}

/// The statement a gate pins, the image leaked to 'static as a pin is.
pub fn statement(air: &WiredMultiGen, program: Vec<u8>) -> Statement {
    let p = pins(air, &program);
    let point = Point { params: p.params, queries: QUERIES, grind_bits: STATEMENT_GRIND_BITS };
    Statement {
        program: Box::leak(program.into_boxed_slice()),
        program_hash: p.program_hash,
        periodic_root: p.periodic_root,
        shape: p.shape,
        extra_blowup_bits: STATEMENT_BLOWUP_BITS,
        points: Box::leak(Box::new([point])),
        words: WORDS,
        max_proof_bytes: 128 * 1024,
    }
}
