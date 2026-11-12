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

use nonos_toolkit::icons::IconId;

use super::icon_path::Mark;
use super::screen::Screen;
use super::sidebar_model::{SideHit, DOWNLOADS, PLACES};

/// The glyph a navigation row wears. Downloads is a directory but reads as a
/// destination, so it keeps the arrow rather than the generic folder, and the
/// capsule store gets the cube it is drawn with everywhere else.
pub fn nav_icon(hit: &SideHit) -> Mark {
    match hit {
        SideHit::Screen(Screen::Home) => Mark::Kit(IconId::Home),
        SideHit::Screen(Screen::Recents) => Mark::Kit(IconId::Clock),
        SideHit::Screen(Screen::Tags) => Mark::Kit(IconId::Tag),
        SideHit::Screen(Screen::Search) => Mark::Kit(IconId::Search),
        SideHit::Screen(_) => Mark::Kit(IconId::FsFolder),
        SideHit::Path(p) if p == DOWNLOADS => Mark::Kit(IconId::Download),
        SideHit::Path(p) if p == PLACES[1].1 => Mark::Kit(IconId::FsFile),
        SideHit::Path(p) if p == PLACES[2].1 => Mark::Cube,
        SideHit::Path(_) => Mark::Kit(IconId::FsFolder),
    }
}
