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

//! Snake's controls at the opening window, the size the hit test reads until a
//! frame lands, at the display scale in force. The table of drawn controls the
//! other proofs walk is here; every point naming the one control drawn there
//! is proven at every scale in `tests/snake_hit_scale.rs`.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

use crate::snake::grid::{WIN_H, WIN_W};
use crate::snake::input::target::Target;
use crate::snake::state::Screen;

pub(crate) use crate::snake_drawn::SCREENS;

/// Every control `s` draws, with the rect its painter draws it in, at the
/// window's opening size, which the hit test reads until a frame lands.
pub(crate) fn drawn(s: Screen) -> Vec<(Target, Rect)> {
    let k = scale();
    crate::snake_drawn::drawn(s, k.px(WIN_W), k.px(WIN_H))
}
