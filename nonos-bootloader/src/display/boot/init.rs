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

use super::layout::splash;
use crate::display::fx::fill_atmosphere;
use crate::display::gop::is_initialized;
use crate::display::ink::palette::{CYAN, TEXT_2, TEXT_3};
use crate::display::ink::{draw_captions, draw_emblem};
use crate::display::version::version_label;

/// The splash: the emblem, lit, and the column the verification log fills.
pub fn init_boot_screen() {
    if !is_initialized() {
        return;
    }
    fill_atmosphere();
    let s = splash();
    draw_emblem(&s.s, 1000, CYAN, true);
    draw_captions(&s.s, b"VERIFYING", TEXT_2, version_label().as_bytes(), TEXT_3);
    draw_log_card();
}

/// The headline, the step list, and the latest log line.
pub fn draw_log_card() {
    super::steps::draw_steps();
    crate::display::log_panel::redraw_all();
}

pub fn reset_animation() {}

pub fn tick_animation() {}
