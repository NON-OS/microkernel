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

use super::refusal::{advice, draw_card, hold};
use crate::display::gop::{get_dimensions, is_initialized};

/// Show why the boot was refused and what to do, then hold the screen until
/// a key or 30 s. The caller then restarts; nothing here changes that.
pub fn show_error_screen(msg: &[u8]) {
    if !is_initialized() {
        return;
    }
    let (w, h) = get_dimensions();
    if w == 0 || h == 0 {
        return;
    }
    let scene = draw_card(&advice(msg), msg);
    hold(&scene);
}
