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

//! A slot's next phase and what to do, from its phase and one poll. A
//! press of the button while it counts down, or while an eject waits on a
//! holder, cancels it; a card that goes away from any phase departs once
//! its absence has held. An empty slot whose latch is open is kept
//! unpowered.

use super::slot::{Act, Phase, Read, Slot};
use super::step_timed::timed;
use super::times::{BUTTON_MS, DEBOUNCE_MS, RETRY_MS};

pub(super) fn next(s: Slot, r: Read, now: u64) -> (Phase, Act) {
    let stay = (s.phase, Act::Nothing);
    let coming = |steady| Phase::Coming { since: now, steady };
    let unpower = if r.powered { Act::PowerOff } else { Act::Nothing };
    let (phase, act) = match s.phase {
        Phase::Empty if r.latch_open => (Phase::Empty, unpower),
        Phase::Empty if r.here() => (coming(Some(now)), Act::Nothing),
        Phase::Empty if r.card() && !r.powered => (coming(None), Act::PowerOn),
        Phase::Empty if r.card() => (coming(None), Act::Nothing),
        Phase::Empty => stay,
        Phase::Coming { since, steady } => timed(r, since, steady, now),
        Phase::Occupied | Phase::Button { .. } | Phase::Leaving { .. } if !r.here() => {
            (Phase::Going { since: now }, Act::Nothing)
        }
        Phase::Occupied if r.pressed() && s.button => {
            (Phase::Button { until: now + BUTTON_MS }, Act::Nothing)
        }
        Phase::Occupied => stay,
        Phase::Going { .. } if r.here() => (Phase::Occupied, Act::Nothing),
        Phase::Going { since } if now.saturating_sub(since) >= DEBOUNCE_MS => {
            (Phase::Empty, Act::Depart)
        }
        Phase::Going { .. } => stay,
        Phase::Button { .. } | Phase::Leaving { .. } if r.pressed() => {
            (Phase::Occupied, Act::Nothing)
        }
        Phase::Button { until } | Phase::Leaving { next: until } if now >= until => {
            (Phase::Leaving { next: now + RETRY_MS }, Act::Eject)
        }
        Phase::Button { .. } | Phase::Leaving { .. } => stay,
        Phase::Off if !r.card() => (Phase::Empty, Act::Nothing),
        Phase::Off if r.pressed() && s.button && !r.latch_open && !r.faulted() => {
            (coming(None), Act::PowerOn)
        }
        Phase::Off => stay,
    };
    // A slot with no power controller is never switched.
    match act {
        Act::PowerOn | Act::PowerOff if !s.power => (phase, Act::Nothing),
        _ => (phase, act),
    }
}
