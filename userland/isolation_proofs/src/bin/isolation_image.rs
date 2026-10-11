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

//! Writes the isolation statement's program.bin and its MANIFEST into the
//! directory given, from the circuit itself. The owner's evaluator reads
//! both; nonos_proof_abi::isolation pins the same numbers.

use std::fmt::Write as _;

use isolation_proofs::image::{encode, pins, record};
use isolation_stark::{shape, Statement, EXTRA_BLOWUP_BITS, GRIND_BITS, QUERIES, WORDS};

fn hex(b: &[u8; 32]) -> String {
    b.iter().fold(String::new(), |mut s, x| {
        let _ = write!(s, "{x:02x}");
        s
    })
}

fn main() {
    let dir = std::env::args().nth(1).expect("usage: isolation_image <dir>");
    let st = Statement { hc: [0; 4], a: [0; 4] };
    let air = shape(&st).expect("the shape");
    let program = encode(&air, &record(&air));
    let p = pins(&air, &program);
    let s = p.shape;
    let manifest = format!(
        "program_hash {}\nperiodic_root {}\nparams {}\nqueries {QUERIES}\n\
         grind_bits {GRIND_BITS}\nextra_blowup_bits {EXTRA_BLOWUP_BITS}\nwords {WORDS}\n\
         trace_width {}\nregion_width {}\nwindow {}\nconstraint_degree {}\n\
         mask_pair {:?}\nchallenge_lanes {}\nprogram_bytes {}\n",
        hex(&p.program_hash),
        hex(&p.periodic_root),
        hex(&p.params),
        s.trace_width,
        s.region_width,
        s.window,
        s.constraint_degree,
        s.mask_pair,
        s.challenge_lanes,
        program.len(),
    );
    std::fs::write(format!("{dir}/program.bin"), &program).expect("program.bin");
    std::fs::write(format!("{dir}/MANIFEST"), manifest).expect("MANIFEST");
}
