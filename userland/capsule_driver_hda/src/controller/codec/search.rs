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

//! The search for a route, shortest first.
//!
//! Linux searches the same graph (`snd_hda_parse_nid_path` in
//! hda_generic.c): from the pin through its connection list, through
//! mixers and selectors, to an analog converter. A selector, or a pin with
//! more than one connection, passes one input at a time, so two outputs may
//! share it only by selecting the same input; a mixer sums all of its inputs
//! and is shared freely. The graph is the codec firmware's word, so each
//! pin's search has a budget of widget visits: mixers that each list 24
//! others and no DAC would otherwise cost millions of visits a pin.

use super::deepen::step;
use super::path::{Path, MAX_DEPTH};
use super::widget::Codec;

/// Connections one pin's search may follow. A real codec's route takes a
/// few dozen; a pin with none on a real codec, a few hundred.
pub const VISIT_BUDGET: u32 = 2048;

/// Why a pin has no route.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Miss {
    /// Every path up to `MAX_DEPTH` widgets was tried; none ends at a DAC.
    NoRoute,
    /// The budget ran out first: a graph no real codec reports.
    OverBudget,
}

/// The shortest path from `pin` to an analog DAC that plays 48 kHz 16-bit,
/// agreeing with every path in `taken` on the inputs shared selectors pass.
pub fn find(codec: &Codec, pin: u8, taken: &[Path]) -> Result<Path, Miss> {
    let mut p = Path { nodes: [0; MAX_DEPTH], sel: [0; MAX_DEPTH], len: 1 };
    p.nodes[0] = pin;
    let mut budget = VISIT_BUDGET;
    for limit in 2..=MAX_DEPTH {
        if step(codec, &mut p, limit, taken, &mut budget) {
            return Ok(p);
        }
        if budget == 0 {
            return Err(Miss::OverBudget);
        }
    }
    Err(Miss::NoRoute)
}
