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

//! A takeover's steps, one atomic operation each. A wait is a step only
//! once DRAWS reads 0: a load that reads more changes nothing, so leaving
//! it out loses no state.

use super::actors::Taker;
use super::events::Event;
use super::gate::Gate;
use super::world::Flags;

/// One step of takeover `t`, `ready` the fate of its claim at the admit;
/// None when it is done or its wait would read DRAWS above 0. A claim made
/// ready latches GONE and then leaves TAKERS, or leaves first when the gate
/// puts the end before the admit.
pub fn taker(g: &Gate, f: &mut Flags, t: &mut Taker, ready: bool) -> Option<Event> {
    let up = if *t == Taker::Start { Taker::Mid } else { Taker::Quiet };
    let (next, e) = match (*t, g.counts_first) {
        (Taker::Start, true) | (Taker::Mid, false) => {
            f.takers += 1;
            (up, Event::Stop)
        }
        (Taker::Mid, true) | (Taker::Start, false) if f.draws == 0 || !g.waits => {
            (up, Event::Quiet)
        }
        (Taker::Quiet, _) if ready && !g.ends_last => {
            f.takers -= u8::from(g.ends);
            (Taker::Freed, Event::End { ready: true })
        }
        (Taker::Quiet, _) if ready => {
            f.gone |= g.sets_gone;
            (Taker::Latched, Event::Latch)
        }
        (Taker::Freed, _) => {
            f.gone |= g.sets_gone;
            (Taker::Ended { ready: true }, Event::Latch)
        }
        (Taker::Quiet | Taker::Latched, _) => {
            let ready = *t == Taker::Latched;
            f.takers -= u8::from(g.ends);
            (Taker::Ended { ready }, Event::End { ready })
        }
        _ => return None,
    };
    *t = next;
    Some(e)
}
