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
        0x3f, 0xcb, 0x74, 0x83, 0x4a, 0x54, 0x03, 0x03, 0x74, 0x42, 0x82, 0x8c, 0xc9, 0x01, 0xd0,
        0x96, 0xea, 0xfb, 0x19, 0x5f, 0xd4, 0x88, 0x88, 0x00, 0x40, 0x6c, 0xd5, 0x0f, 0xc1, 0xae,
        0x03, 0x62,
    ],
    periodic_root: [
        0x2f, 0xbd, 0x5a, 0x47, 0x9a, 0xaa, 0xb2, 0x19, 0xef, 0x34, 0x5b, 0x11, 0xb1, 0x1f, 0x84,
        0x5c, 0x1b, 0x50, 0xea, 0xf6, 0xfb, 0x87, 0xdb, 0x81, 0x17, 0x22, 0x65, 0x32, 0xf5, 0x6f,
        0x6c, 0x6b,
    ],
    shape: Shape {
        trace_width: 37,
        region_width: 29,
        window: 2,
        constraint_degree: 8,
        mask_pair: Some((35, 36)),
        challenge_lanes: 2,
    },
    extra_blowup_bits: 5,
    points: &[Point {
        params: [
            0xea, 0x50, 0x1b, 0xb1, 0x09, 0x86, 0x6b, 0x36, 0x86, 0x95, 0x31, 0x36, 0x7e, 0xb7,
            0x24, 0x05, 0xa7, 0xf0, 0x2b, 0x17, 0xcf, 0xab, 0x28, 0x72, 0x6c, 0x2e, 0x19, 0x56,
            0x6a, 0xa0, 0x8c, 0x44,
        ],
        queries: 26,
        grind_bits: 28,
    }],
    words: 8,
    max_proof_bytes: 128 * 1024,
};
