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

//! The video player asks for full screen while it shows a film
//! (app/full_screen.rs), and nowhere else.

use crate::ui::screen::Route;
use crate::video_full_screen::wants_full_screen;

#[test]
fn a_film_in_the_player_is_watched_full_screen() {
    assert!(wants_full_screen(Route::Player, true));
}

#[test]
fn the_lists_and_an_empty_player_keep_the_window() {
    assert!(!wants_full_screen(Route::Player, false));
    for route in [Route::Library, Route::Files, Route::Details] {
        assert!(!wants_full_screen(route, true));
    }
}
