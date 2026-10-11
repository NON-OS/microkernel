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

//! The byte writers, and the transition tape: each op its tag, then a
//! constant's two limbs, an input's index or an operation's two sources.

use nonos_stark::field::Fp;
use stark_proofs::wrap::Op;

use super::record::Recorded;

pub(super) fn u16s(out: &mut Vec<u8>, v: usize) {
    out.extend_from_slice(&u16::try_from(v).expect("a count past u16").to_be_bytes());
}

pub(super) fn fp(out: &mut Vec<u8>, v: Fp) {
    out.extend_from_slice(&v.to_u64().to_be_bytes());
}

pub(super) fn ops(out: &mut Vec<u8>, r: &Recorded) {
    for (i, op) in r.ops.iter().enumerate() {
        match *op {
            Op::Const => {
                out.push(0);
                fp(out, r.vals[i].c0);
                fp(out, r.vals[i].c1);
            }
            Op::Input(k) => {
                out.push(1);
                u16s(out, k);
            }
            Op::Add(a, b) | Op::Sub(a, b) | Op::Mul(a, b) => {
                out.push(match op {
                    Op::Add(..) => 2,
                    Op::Sub(..) => 3,
                    _ => 4,
                });
                u16s(out, a as usize);
                u16s(out, b as usize);
            }
            Op::Inv(_) => panic!("an inversion in the transitions: no image can carry it"),
        }
    }
}
