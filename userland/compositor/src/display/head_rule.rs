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
//! again, as one whose FLUSH was refused otherwise does; no answer leaves it
//! be until the next poll.

use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::State;

use super::head::Head;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadStep {
    Keep,
    Repaint,
    /// A FLUSH it refused, not for a stale primary: the whole screen again.
    Resend,
    Resurface,
}

pub fn head_step(h: &Head, state: Option<State>) -> HeadStep {
    match state {
        Some(s) if s.serial != h.serial || h.stale => HeadStep::Resurface,
        Some(s) if s.flags & LOST != 0 => HeadStep::Repaint,
        _ if h.stale => HeadStep::Resurface,
        _ if h.owed => HeadStep::Resend,
        _ => HeadStep::Keep,
    }
}

/// How many mirrored outputs `outputs` asks for, the wire server keeping
/// four at most: a poll that holds fewer primes them again.
pub fn heads_wanted(outputs: u16) -> usize {
    outputs.clamp(1, 4) as usize - 1
}

/// The mirrored outputs `outputs` asks for that `have` does not hold yet,
/// in order. A poll primes only these, so a head that would not answer is
/// asked again without the heads already shown let go and the whole screen
/// sent to every one of them each time.
pub fn heads_missing(outputs: u16, have: &[u16]) -> impl Iterator<Item = u16> + '_ {
    let last = u16::try_from(heads_wanted(outputs)).unwrap_or(0).saturating_add(1);
    (1..last).filter(move |o| !have.contains(o))
}

/// Where a head for `output` goes in `have`, kept in output order.
pub fn head_slot(output: u16, have: &[u16]) -> usize {
    have.iter().position(|&o| o > output).unwrap_or(have.len())
}
