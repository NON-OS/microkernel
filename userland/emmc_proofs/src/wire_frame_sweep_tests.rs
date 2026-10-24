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

//! Request headers and bodies at every edge: a header is taken exactly when
//! it is 20 bytes or more under the wire's magic and version, its fields
//! little-endian where PLAN.md section 4 puts them; a body exactly when it
//! is the payload_len the header names, data only on WRITE.

use nonos_blk_wire::{body_fits, decode_request, OP_WRITE};

use crate::sweep_rng::{rounds, Rng};

const MAGIC: u32 = 0x4E42_5731;

#[test]
fn a_header_is_taken_exactly_as_the_wire_names_it() {
    let (mut taken, mut refused) = (0u32, 0u32);
    for seed in 1..rounds(200_000) {
        let mut r = Rng::new(seed);
        let len = r.edge(&[0, 1, 19, 20, 21, 32, 64]) as usize % 4097;
        let mut b: Vec<u8> = (0..len).map(|_| r.next() as u8).collect();
        if len >= 6 && r.below(8) != 0 {
            b[..4].copy_from_slice(&MAGIC.to_le_bytes());
            let version: u16 = if r.below(8) == 0 { 2 + r.below(100) as u16 } else { 1 };
            b[4..6].copy_from_slice(&version.to_le_bytes());
        }
        let u16_at = |at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
        let u32_at = |at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
        let ours = len >= 20 && u32_at(0) == MAGIC && u16_at(4) == 1;
        let got = decode_request(&b);
        assert_eq!(got.is_some(), ours, "seed {seed}: {len} bytes");
        if let Some(q) = got {
            let want = (u16_at(6), u16_at(8), u16_at(10), u32_at(12), u32_at(16));
            assert_eq!((q.op, q.flags, q.unit, q.request_id, q.payload_len), want, "seed {seed}");
        }
        (taken, refused) = (taken + ours as u32, refused + !ours as u32);
    }
    assert!(cfg!(miri) || taken.min(refused) >= 40_000, "taken {taken}, refused {refused}");
}

#[test]
fn a_body_is_taken_exactly_when_it_is_the_one_the_header_names() {
    let mut header = [0u8; 20];
    header[..4].copy_from_slice(&MAGIC.to_le_bytes());
    header[4] = 1;
    let mut seen = [0u32; 4];
    for seed in 1..rounds(200_000) {
        let mut r = Rng::new(seed);
        let op = r.below(8) as u16;
        let body = r.edge(&[0, 1, 11, 12, 13, 12 + 512, 12 + 4096]) as usize % (1 << 20);
        let named =
            r.edge(&[body as u64, body as u64 + 1, body.saturating_sub(1) as u64, u32::MAX as u64])
                as u32;
        header[6..8].copy_from_slice(&op.to_le_bytes());
        header[16..20].copy_from_slice(&named.to_le_bytes());
        let q = decode_request(&header).unwrap();
        let rule = body as u64 == u64::from(named) && (op == OP_WRITE || body <= 12);
        assert_eq!(body_fits(&q, body), rule, "seed {seed}: op {op}, {body} bytes, {named} named");
        seen[(rule as usize) << 1 | (op == OP_WRITE) as usize] += 1;
    }
    assert!(cfg!(miri) || seen.iter().all(|&n| n >= 5_000), "{seen:?}");
}
