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

//! What Rust reads and writes of a suspend context, by the offsets in
//! `asm/suspend.h`; the asserts below and the proofs hold the two equal.

/// Words before the vector registers: through FPSR, then one pad word.
pub const WORDS: usize = 46;

/// Word indices, each `CTX_*` / 8.
pub const MAGIC: usize = 0;
pub const MPIDR: usize = 1;
pub const FLAGS: usize = 2;

/// `CTX_ARMED`: a context its CPU armed for this call.
pub const ARMED: u64 = 0x4e4f_4e4f_5352_534d;

/// `FLAG_*` bits Rust sets. FP is the save's own (bit 0).
pub const FLAG_GICV3: u64 = 1 << 1;
pub const FLAG_PAN: u64 = 1 << 2;
pub const FLAG_SVE: u64 = 1 << 3;
pub const FLAG_MTE2: u64 = 1 << 4;
pub const FLAG_SSBS: u64 = 1 << 5;

/// `SUSPEND_RESUMED`: the core powered down and came back.
pub const RESUMED: i64 = 1;

/// `CTX_Q` and `CTX_SIZE`.
pub const Q_OFFSET: usize = 368;
pub const SIZE: usize = 880;
