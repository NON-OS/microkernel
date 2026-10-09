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

//! Which build of the chat program this CPU should run.
//!
//! The shipped program is built for x86-64-v3 (AVX2, FMA, F16C, BMI1/2,
//! LZCNT, MOVBE), which a CPU without them stops on an invalid opcode. A
//! build for x86-64-v2 ships beside it for such a CPU, and a plain x86-64
//! build for QEMU's software CPU, which runs it fastest. The order, and
//! what is said for each, is chat_pick's, from what CPUID says here.

use alloc::vec::Vec;

/// The builds of `program` this CPU can run, best first, each with the
/// line said when it is chosen, if any. A tier starts the first one the
/// store holds.
pub fn choices(program: &'static [u8]) -> Vec<(&'static [u8], &'static [u8])> {
    super::chat_pick::pick(program, super::tcg::under_tcg(), super::isa::runs_v3())
}
