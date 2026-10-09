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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    Library,
    Files,
    Player,
    Details,
}

// The player keeps no watch history, playlists or settings of its own, so it
// offers the two pages its catalogue can fill: every video, and by folder.
pub const NAV: [Route; 2] = [Route::Library, Route::Files];

impl Route {
    pub fn label(self) -> &'static str {
        match self {
            Route::Library => "Library",
            Route::Files => "Folders",
            Route::Player => "Now Playing",
            Route::Details => "Media Details",
        }
    }

    pub fn in_nav(self) -> bool {
        NAV.contains(&self)
    }

    pub fn chrome(self) -> bool {
        !matches!(self, Route::Player)
    }
}
