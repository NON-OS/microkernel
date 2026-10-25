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

/*
 * Whether a guarded screen's presses show armed (look.rs): the one fn the
 * paint asks to draw them greyed or not, and the press asks after one more
 * look at focus (look::pressed), so the two can never disagree. Focus is
 * what the last look kept: lost, `since` is gone and the presses grey.
 */

use super::look::Arm;
use super::moves::{moves, Shown};
use crate::wallet::order::terms::arming::armed;
use crate::wallet::screen::hits::Press;

/// Whether `shown`'s guarded presses are armed on `a` at `now`.
pub fn shows_armed(a: &Arm, shown: Option<Shown>, now: u64) -> bool {
    let g = a.guard;
    let held = g.since.is_some_and(|since| armed(g.drawn, true, since, now));
    shown.is_some() && a.shown == shown && held
}

/// Whether `press` on `shown` (None: Enter) is drawn greyed at `now`: it
/// moves value or signs, and its screen's guard is not armed.
pub fn greys(a: &Arm, shown: Option<Shown>, press: Option<Press>, now: u64) -> bool {
    shown.is_some_and(|s| moves(s, press)) && !shows_armed(a, shown, now)
}

/// After a look: the armed state the frame now shows, and whether it
/// differs from `lit`, the one last shown, so the frame is painted again.
pub fn relit(lit: bool, a: &Arm, shown: Option<Shown>, now: u64) -> (bool, bool) {
    let now_lit = shows_armed(a, shown, now);
    (now_lit, now_lit != lit)
}
