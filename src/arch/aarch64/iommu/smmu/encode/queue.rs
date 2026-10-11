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

//! Command and event queue indices. A PROD or CONS value is an index in the
//! low `log2` bits with a wrap bit above it: equal indices mean empty when
//! the wrap bits agree and full when they differ.

pub const fn index(value: u32, log2: u8) -> u32 {
    value & ((1 << log2) - 1)
}

const fn wrap(value: u32, log2: u8) -> u32 {
    value & (1 << log2)
}

pub const fn is_empty(prod: u32, cons: u32, log2: u8) -> bool {
    index(prod, log2) == index(cons, log2) && wrap(prod, log2) == wrap(cons, log2)
}

pub const fn is_full(prod: u32, cons: u32, log2: u8) -> bool {
    index(prod, log2) == index(cons, log2) && wrap(prod, log2) != wrap(cons, log2)
}

/// The next value after `value`. Index and wrap bit are one counter of
/// `log2 + 1` bits, so carrying out of the index flips the wrap bit.
pub const fn advance(value: u32, log2: u8) -> u32 {
    (value + 1) & ((2 << log2) - 1)
}

/// Whether CONS has moved past `target`, a PROD value the kernel wrote: the
/// unit has consumed every command up to it. Works across one wrap, which
/// is all a queue can be ahead by.
pub const fn consumed(cons: u32, target: u32, log2: u8) -> bool {
    let span = 2 << log2;
    let behind = (target.wrapping_sub(cons)) & (span - 1);
    behind == 0 || behind > (1 << log2)
}
