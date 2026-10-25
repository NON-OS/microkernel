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

//! What a poll does with one mirrored output, from its STATE: a new
//! serial, or a FLUSH that found its primary stale, asks SURFACE again; a
//! LOST output, set again by its driver, has the whole screen sent to it
//! again; no answer leaves it be until the next poll.

use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::State;

use super::head::Head;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadStep {
    Keep,
    Repaint,
    Resurface,
}

pub fn head_step(h: &Head, state: Option<State>) -> HeadStep {
    match state {
        Some(s) if s.serial != h.serial || h.stale => HeadStep::Resurface,
        Some(s) if s.flags & LOST != 0 => HeadStep::Repaint,
        _ if h.stale => HeadStep::Resurface,
        _ => HeadStep::Keep,
    }
}

/// How many mirrored outputs `outputs` asks for, the wire server keeping
/// four at most: a poll that holds fewer primes them again.
pub fn heads_wanted(outputs: u16) -> usize {
    outputs.clamp(1, 4) as usize - 1
}
