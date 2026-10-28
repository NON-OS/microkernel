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

//! How much a window (window.rs) holds at once: a few GETs and a bounded
//! number of bytes, one package larger than that alone. Pure, so the host
//! proofs hold it.

/// No index or package is larger (http/get.rs MAX_BODY).
const MAX: usize = 160 << 20;
/// At most this many GETs in flight, and their replies held, at once...
const WIDTH: usize = 3;
/// ...and at most this many bytes, and a sixth of the heap left free (an
/// unpack and its pack take several times a package), unless one package
/// alone is larger, which then comes and is placed with nothing beside it.
const HELD: usize = 64 << 20;
const SHARE: usize = 6;
/// Room for a reply's status and headers beside its body.
const HEAD: usize = 16 << 10;

/// The most a reply may bring for a package of `size` in the index.
pub fn cap(size: usize) -> usize {
    if size == 0 {
        MAX
    } else {
        size.saturating_add(HEAD).min(MAX)
    }
}

/// Whether GET `n` may start beside `held`, those in flight or waiting,
/// with `free` bytes of heap not in use.
pub fn room(sizes: &[usize], held: &[usize], n: usize, free: usize) -> bool {
    let bytes = held.iter().fold(cap(sizes[n]), |sum, &at| sum.saturating_add(cap(sizes[at])));
    held.is_empty() || (held.len() < WIDTH && bytes <= HELD.min(free / SHARE))
}
