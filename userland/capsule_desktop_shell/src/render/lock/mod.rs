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

//! The lock screen and the passphrase prompt, drawn on the chrome. The lock
//! screen is opaque over the whole canvas, so no window's pixels are ever
//! under it to show through, and it is the only thing drawn while it is up:
//! no menu, toast or prompt.

mod field;
mod prompt;
mod screen;

pub use prompt::paint as paint_prompt;
pub use screen::paint as paint_screen;

use crate::render::layout::Rect;

fn rect(r: (u32, u32, u32, u32)) -> Rect {
    Rect { x: r.0, y: r.1, width: r.2, height: r.3 }
}
