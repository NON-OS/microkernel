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


//! A HashX program: up to 512 register instructions over eight 64-bit
//! registers, drawn from a seed so that every seed gives a different function.
//!
//! Generation simulates an Ivy Bridge core cycle by cycle and keeps only
//! programs that fill it exactly: 512 instructions, 192 multiplications and a
//! critical path of 195 cycles. A seed whose program misses any of the three
//! is refused, which happens for fewer than one seed in ten thousand.

mod generate;
mod schedule;
mod template;

pub use generate::generate;

/// Instructions in a full program; programs that are shorter are refused.
pub const PROGRAM_SIZE: usize = 512;

/// The instruction set, in the reference's order. The order is load bearing:
/// the first three are the multiplications, and the generator compares
/// instruction groups by identity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    UmulhR,
    SmulhR,
    MulR,
    SubR,
    XorR,
    AddRs,
    RorC,
    AddC,
    XorC,
    Target,
    Branch,
}

impl Op {
    pub fn is_mul(self) -> bool {
        matches!(self, Op::UmulhR | Op::SmulhR | Op::MulR)
    }
}

/// One instruction as the interpreter needs it. `src` and `dst` are register
/// numbers below eight; an instruction that has no such operand carries zero
/// there and never reads it.
#[derive(Clone, Copy, Debug)]
pub struct Instr {
    pub op: Op,
    pub src: u8,
    pub dst: u8,
    pub imm: u32,
}

/// A generated program, always exactly `PROGRAM_SIZE` instructions.
pub struct Program {
    pub code: [Instr; PROGRAM_SIZE],
}
