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

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Ink, Label};

use crate::snake::state::kept::line;
use crate::snake::state::Game;
use crate::snake::ui::rank_geom::kept_note;
use crate::snake::ui::text::as_str;

// Beside Back: whether the ranks reached the disk, stayed in memory until
// power off, or were not written or read, with the reason in the vfs's words.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (caption, why, warn) = line(game.kept);
    if caption.is_empty() {
        return;
    }
    let mut said = String::from(as_str(caption));
    said.push_str(as_str(why));
    let ink = if warn { Ink::Warn } else { Ink::Muted };
    Label::new(&said).ink(ink).draw(fb, kept_note(fb.width, fb.height));
}
