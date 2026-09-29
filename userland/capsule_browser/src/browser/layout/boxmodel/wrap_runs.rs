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

use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::abs_out_of_flow::out_of_flow;
use super::flush_run::flush_run;
use super::tree::BoxNode;

/* Rewrap a child list so every contiguous inline run sits in one anonymous
 * block and block-level children pass through in order. An out-of-flow box
 * passes through too: wrapped, it would make an empty in-flow block (a flex
 * item taking a gap) around something that takes no space. The lists are
 * sized exactly, counted first: a box tree holds tens of thousands of them
 * and growth by doubling would leave half of each unused. */
pub(super) fn wrap_runs(parent: &Computed, children: Vec<BoxNode>) -> Vec<BoxNode> {
    let passes = |c: &BoxNode| c.kind.block_level() || out_of_flow(&c.style);
    let mut runs: Vec<usize> = Vec::new();
    let mut blocks = 0;
    for (i, c) in children.iter().enumerate() {
        if passes(c) {
            blocks += 1;
        } else if let Some(n) = runs.last_mut().filter(|_| i > 0 && !passes(&children[i - 1])) {
            *n += 1;
        } else {
            runs.push(1);
        }
    }
    let mut out: Vec<BoxNode> = Vec::with_capacity(blocks + runs.len());
    let mut lens = runs.into_iter();
    let mut run: Vec<BoxNode> = Vec::new();
    for c in children {
        if passes(&c) {
            flush_run(&mut out, &mut run, parent);
            out.push(c);
        } else {
            if run.is_empty() {
                run.reserve_exact(lens.next().unwrap_or(0));
            }
            run.push(c);
        }
    }
    flush_run(&mut out, &mut run, parent);
    out
}
