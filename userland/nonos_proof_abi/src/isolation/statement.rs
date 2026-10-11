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

//! The isolation statement a gate pins: plan/12's program image, written
//! from isolation_stark's circuit by isolation_proofs' isolation_image, and
//! the numbers the image does not carry. isolation_proofs checks every one
//! against the circuit.

use nox_verify::{Point, Shape, Statement};

pub static STATEMENT: Statement = Statement {
    program: include_bytes!("program.bin"),
    program_hash: [
        0x3b, 0x81, 0x64, 0x43, 0x69, 0x36, 0xd5, 0x6d, 0x71, 0x5e, 0x02, 0xf5, 0xf3, 0x93, 0x92,
        0x1a, 0xc9, 0xaa, 0xc0, 0x32, 0xa7, 0x21, 0x9c, 0xf5, 0x60, 0xd1, 0x15, 0xde, 0xdb, 0x4f,
        0xe9, 0xf3,
    ],
    periodic_root: [
        0x00, 0x6b, 0xf7, 0x61, 0x13, 0x98, 0xdf, 0xd3, 0x65, 0x4f, 0x29, 0x1e, 0x59, 0x58, 0x42,
        0x82, 0xd8, 0xb5, 0xaa, 0xcc, 0xee, 0xb9, 0x4e, 0xd0, 0xc4, 0xb1, 0x5a, 0x12, 0xa9, 0xc5,
        0x51, 0x49,
    ],
    shape: Shape {
        trace_width: 35,
        region_width: 29,
        window: 2,
        constraint_degree: 11,
        mask_pair: Some((33, 34)),
        challenge_lanes: 2,
    },
    extra_blowup_bits: 5,
    points: &[Point {
        params: [
            0x4b, 0x8a, 0x09, 0xbf, 0x73, 0x9a, 0xb8, 0x71, 0x49, 0xb8, 0x07, 0x53, 0x40, 0x3f,
            0x0f, 0x8a, 0xd9, 0xbd, 0x1a, 0xdd, 0x23, 0xb4, 0x7a, 0xbf, 0x09, 0x33, 0x0a, 0x4f,
            0x95, 0x6a, 0xa1, 0xc3,
        ],
        queries: 26,
        grind_bits: 28,
    }],
    words: 8,
    max_proof_bytes: 128 * 1024,
};
