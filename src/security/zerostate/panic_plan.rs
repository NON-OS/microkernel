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

//! Which bytes the panic wipe zeroes, worked out apart from the wipe itself
//! so the host proofs can walk every case (kernel_proofs `panic_plan`).
//!
//! The wipe covers the heap and must never cover what the panic path runs
//! on: its own stack and the kernel image, code and statics among them. The
//! heap is a window of its own and does not overlap either today, so the
//! subtraction normally leaves the heap whole; it is here so that a change in
//! the layout costs a smaller wipe, never a wipe of the running stack.
//! Nothing here allocates: the result is written into a fixed array.

/// A half-open byte range `[start, end)`; empty when `end <= start`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start: u64,
    pub(crate) end: u64,
}

/// The most spans a plan can come to: each kept span splits at most one
/// span in two, so this holds `wipe` minus up to MAX_KEEP spans.
pub(crate) const MAX_KEEP: usize = 7;
pub(crate) const MAX_SPANS: usize = MAX_KEEP + 1;

/// `wipe` minus every span in `keep`, in address order, into `out`. Returns
/// how many spans were written, or `None` when `keep` holds more than
/// MAX_KEEP spans, in which case nothing may be wiped.
pub(crate) fn plan(wipe: Span, keep: &[Span], out: &mut [Span; MAX_SPANS]) -> Option<usize> {
    if keep.len() > MAX_KEEP {
        return None;
    }
    let mut n = 0;
    if wipe.end > wipe.start {
        out[0] = wipe;
        n = 1;
    }
    for k in keep.iter().filter(|k| k.end > k.start) {
        let mut next = [Span { start: 0, end: 0 }; MAX_SPANS];
        let mut m = 0;
        for s in &out[..n] {
            // The part of `s` below `k`, then the part above it.
            for part in [
                Span { start: s.start, end: s.end.min(k.start) },
                Span { start: s.start.max(k.end), end: s.end },
            ] {
                if part.end > part.start {
                    next[m] = part;
                    m += 1;
                }
            }
        }
        out[..m].copy_from_slice(&next[..m]);
        n = m;
    }
    Some(n)
}
