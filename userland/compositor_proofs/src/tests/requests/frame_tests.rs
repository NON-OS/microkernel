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

//! Whole frames, header and body, with lying lengths: a body reaches a
//! decoder only when the header's length is the frame's, and then it is
//! read exactly as on its own.

use super::hostile::{body, word, words};
use super::rng::Rng;
use super::sweep_tests::all;
use crate::protocol::{parse, HDR_LEN, MAGIC};
use crate::tests::wire::parse_frames::frame;

#[test]
fn a_body_reaches_its_decoder_only_whole() {
    let ops = all();
    let mut r = Rng::new(0x4E43_F4A3);
    let (mut whole, mut refused, mut taken) = (0usize, 0usize, 0usize);
    for _ in 0..200_000 {
        let (name, len, good, decode, model) = ops[r.range(0, ops.len() as u32 - 1) as usize];
        let x = body(&mut r, len, good);
        let said = if r.one_in(4) { word(&mut r) } else { x.len() as u32 };
        let mut f = frame(r.word() as u16, r.word() as u16, r.word(), said, &x);
        match r.range(0, 9) {
            0 => f.truncate(r.range(0, f.len() as u32) as usize),
            1 => f.extend_from_slice(&r.word().to_le_bytes()[..r.range(1, 4) as usize]),
            2 => f[..4].copy_from_slice(&word(&mut r).to_le_bytes()),
            _ => {}
        }
        let Ok((_, got)) = parse(&f) else {
            refused += 1;
            continue;
        };
        let fits = f.len() >= HDR_LEN && u32::from_le_bytes(f[..4].try_into().unwrap()) == MAGIC;
        assert!(fits && got.len() == f.len() - HDR_LEN, "{name}");
        assert_eq!(got.len() as u64, u32::from_le_bytes(f[16..20].try_into().unwrap()) as u64);
        let want = (got.len() == len).then(|| model(&words(got))).flatten();
        assert_eq!(decode(got), want, "{name} {got:02X?}");
        whole += 1;
        taken += want.is_some() as usize;
    }
    assert!(whole >= 50_000 && refused >= 50_000 && taken >= 20_000, "{whole} {refused} {taken}");
}
