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

/* A frame of the full-screen installer, and the request ids it is sent under. */

use super::surface::Surface;
use crate::install::state::{Screen, State};
use crate::install::ui::full::paint;

/* A disk is being written or read back. */
pub(super) fn busy(state: &State) -> bool {
    matches!(state.screen, Screen::Writing | Screen::Verifying)
}

pub(super) fn draw(state: &mut State, s: &Surface, rid: &mut u32) {
    paint(state, &mut s.buffer());
    s.commit(next(rid));
}

pub(super) fn next(rid: &mut u32) -> u32 {
    *rid = rid.wrapping_add(1).max(4);
    *rid
}
