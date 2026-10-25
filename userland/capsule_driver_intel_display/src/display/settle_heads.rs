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
//! The takeover settled from the heads it made (G-78): each head's pipe is
//! taken, and its two surfaces are the runs of the GGTT kept; everything
//! else a killed driver may have left goes (`engine::settle`).

use super::frames::Frames;
use super::head::Head;
use super::intel::MAX_HEADS;
use crate::engine::access::{Expiry, Mmio};
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::Pipe;
use crate::engine::settle::{settle, Run};
use crate::engine::takeover::Output;

/// `lit` as read at takeover, `heads` as taken from it, `ours` the plan's.
pub fn settle_heads<F: Frames>(
    m: &mut dyn Mmio,
    lit: &[Option<Output>],
    heads: &[Option<Head<F>>],
    (t, ours, total): (Pte, u32, u32),
    until: &dyn Expiry,
) {
    let (mut pipes, mut keep) = ([Pipe::A; MAX_HEADS], [(0, 0); 2 * MAX_HEADS]);
    let mut n = 0;
    for h in heads.iter().flatten().take(MAX_HEADS) {
        let s = &h.scan;
        pipes[n] = s.pipe;
        let run = |g: u32| -> Run { (g / 4096, s.pages) };
        (keep[2 * n], keep[2 * n + 1]) = (run(s.surfaces[0]), run(s.surfaces[1]));
        n += 1;
    }
    settle(m, (lit, &pipes[..n]), &keep[..2 * n], (t, ours, total), until);
}
