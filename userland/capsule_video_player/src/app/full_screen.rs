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

//! A film is watched full screen: the window asks for it (the runner takes
//! it the way the green button does, the dock hidden) while the player shows
//! a film, playing or paused, and gives it back on the way out of the player.
//! Pausing does not ask it back, so the window does not jump on every pause.

use crate::ui::screen::Route;

pub fn wants_full_screen(route: Route, film_open: bool) -> bool {
    route == Route::Player && film_open
}
