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

//! The ring's take, pure (K8): each event handed out is cleared from its
//! slot. A drained keystroke stayed in the ring's buffer until a later post
//! overwrote it, up to 1023 events on, readable by anything that could read
//! kernel memory meanwhile. Proven in kernel_proofs::input_take.

/// Move events from `tail` up to `head` into `out`, leaving each slot taken
/// at its default. Returns how many; `tail` advances as ring_math::wrap
/// does, modulo the buffer's length.
pub(crate) fn take<E: Copy + Default>(
    buf: &mut [E],
    head: usize,
    tail: &mut usize,
    out: &mut [E],
) -> usize {
    let mut n = 0;
    while n < out.len() && *tail != head && *tail < buf.len() {
        out[n] = core::mem::take(&mut buf[*tail]);
        *tail = (*tail + 1) % buf.len();
        n += 1;
    }
    n
}
