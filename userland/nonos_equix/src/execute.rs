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


//! The interpreter. HashX is meant to be compiled to machine code, but the
//! capsule never maps memory executable, and the reference's interpreter
//! computes the same function.

use crate::program::{Op, Program};

fn umulh(a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) >> 64) as u64
}

fn smulh(a: u64, b: u64) -> u64 {
    ((i128::from(a as i64) * i128::from(b as i64)) >> 64) as u64
}

/// Constants are 32 bits in the program and sign extended into a register.
fn sign_extend(x: u32) -> u64 {
    x as i32 as i64 as u64
}

/// Runs `program` over the registers in place.
///
/// The one branch per program is taken at most once: it jumps back to the
/// instruction after the last TARGET when the low 32 bits of the last high
/// multiplication have none of the branch's four mask bits set.
pub fn execute(program: &Program, r: &mut [u64; 8]) {
    let mut target = 0usize;
    let mut branch_enable = true;
    let mut result = 0u32;
    let mut i = 0usize;
    while i < program.code.len() {
        let instr = &program.code[i];
        let dst = usize::from(instr.dst & 7);
        let src = usize::from(instr.src & 7);
        match instr.op {
            Op::UmulhR => {
                r[dst] = umulh(r[dst], r[src]);
                result = r[dst] as u32;
            }
            Op::SmulhR => {
                r[dst] = smulh(r[dst], r[src]);
                result = r[dst] as u32;
            }
            Op::MulR => r[dst] = r[dst].wrapping_mul(r[src]),
            Op::SubR => r[dst] = r[dst].wrapping_sub(r[src]),
            Op::XorR => r[dst] ^= r[src],
            Op::AddRs => r[dst] = r[dst].wrapping_add(r[src] << (instr.imm & 3)),
            Op::RorC => r[dst] = r[dst].rotate_right(instr.imm & 63),
            Op::AddC => r[dst] = r[dst].wrapping_add(sign_extend(instr.imm)),
            Op::XorC => r[dst] ^= sign_extend(instr.imm),
            Op::Target => target = i,
            Op::Branch => {
                if branch_enable && result & instr.imm == 0 {
                    i = target;
                    branch_enable = false;
                }
            }
        }
        i += 1;
    }
}
