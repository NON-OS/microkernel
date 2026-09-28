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

//! SM, RM, DECSET and DECRST, each applied to every parameter given.

use super::mouse_mode::MouseMode;
use super::state::Term;
use crate::params::Params;

impl Term {
    pub(super) fn set_ansi_modes(&mut self, p: &Params, on: bool) {
        for g in p.groups() {
            match g[0] {
                4 => self.modes.insert = on,
                20 => self.modes.newline = on,
                _ => {}
            }
        }
    }

    pub(super) fn set_dec_modes(&mut self, p: &Params, on: bool) {
        for g in p.groups() {
            self.set_dec_mode(g[0], on);
        }
    }

    fn set_dec_mode(&mut self, mode: u16, on: bool) {
        let m = &mut self.modes;
        match mode {
            1 => m.cursor_keys = on,
            5 => {
                m.reverse_video = on;
                self.touch_all();
            }
            6 => {
                m.origin = on;
                let home = if on { self.scr_ref().top } else { 0 };
                let cur = &mut self.scr().cur;
                cur.x = 0;
                cur.y = home;
                cur.pending_wrap = false;
            }
            7 => m.autowrap = on,
            12 => m.cursor_blink = on,
            25 => m.cursor_visible = on,
            66 => m.keypad = on,
            9 => m.mouse = if on { MouseMode::Press } else { MouseMode::Off },
            1000 => m.mouse = if on { MouseMode::Click } else { MouseMode::Off },
            1002 => m.mouse = if on { MouseMode::Drag } else { MouseMode::Off },
            1003 => m.mouse = if on { MouseMode::Motion } else { MouseMode::Off },
            1004 => m.focus = on,
            1006 => m.mouse_sgr = on,
            1007 => m.alt_scroll = on,
            2004 => m.bracketed_paste = on,
            2026 => m.sync = on,
            47 | 1047 | 1048 | 1049 => self.set_screen_mode(mode, on),
            _ => {}
        }
    }
}
