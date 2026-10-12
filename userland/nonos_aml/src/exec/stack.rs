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

//! How much of the stack an evaluation has used: the distance from where
//! it began to where the current call is. Stacks grow down on every target
//! NONOS builds (x86-64, AArch64, RISC-V), so the difference is the bytes
//! in use. The address is of a local kept on the stack by `black_box`; it
//! is compared, never dereferenced.

/// An address in the caller's frame.
#[inline(never)]
pub fn here() -> usize {
    let marker = 0u8;
    core::hint::black_box(&marker) as *const u8 as usize
}

/// Bytes used since `top`, 0 if the stack somehow sits above it.
pub fn used_since(top: usize) -> usize {
    top.saturating_sub(here())
}
