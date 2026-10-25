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

//! A covering claim in the takeover model (L79), in claim.rs's order: its
//! entry made, the console stopped and the draws under way waited out,
//! bootfb's claim read and taken back (take_back.rs) until `next` says done,
//! the admit (taker_admit.rs), the takeover's end, a refused claim withdrawn.

use super::knob::Knob;
use super::types::Stage;
use super::world::{code, END};
use super::world_state::World;

pub(super) const STOP: u8 = 1;
const QUIET: u8 = 2;
pub(super) const PEEK: u8 = 3;
pub(super) const ADMIT: u8 = 11;
pub(super) const ENDS: u8 = 12;
pub(super) const WITHDRAW: u8 = 13;

pub(super) fn taker(w: &mut World, g: usize, k: Knob) -> bool {
    match w.pc[g] {
        0 => {
            w.gpu[g] = Some(code(Stage::Making));
            let first = matches!(k, Knob::Control | Knob::TakeAfterAdmit);
            w.pc[g] = if first { ADMIT } else { STOP };
        }
        // stop_drawing: counted, or (TakersFlag) one flag for every takeover.
        STOP => {
            w.takers = if k == Knob::TakersFlag { 1 } else { w.takers + 1 };
            w.gone |= k == Knob::GoneEarly;
            w.pc[g] = QUIET;
        }
        QUIET if w.draws > 0 => return false,
        QUIET => w.pc[g] = PEEK,
        PEEK => return super::taker_peek::peek(w, g, k),
        ADMIT => return super::taker_admit::admit(w, g, k),
        // end_takeover, whatever the admit answered.
        ENDS => {
            w.takers = if k == Knob::TakersFlag { 0 } else { w.takers - 1 };
            w.pc[g] = if w.g[g].ok { END } else { WITHDRAW };
        }
        WITHDRAW => (w.gpu[g], w.pc[g]) = (None, END),
        pc => return super::take_back::step(w, g, pc, k),
    }
    true
}
