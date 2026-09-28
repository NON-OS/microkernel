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

//! Routing an OSC by its number, and the title it most often sets.

use alloc::string::String;

use crate::limits::{MAX_TITLE, MAX_TITLE_STACK};
use crate::term::state::Term;

/// Text a program sent, kept only as far as it is printable: a title or a
/// path cannot smuggle control bytes into the host's own display.
pub(in crate::term) fn clean_text(b: &[u8], max: usize) -> String {
    let s = String::from_utf8_lossy(b);
    s.chars().filter(|c| !c.is_control()).take(max).collect()
}

impl Term {
    pub(in crate::term) fn osc_dispatch(&mut self, data: &[u8]) {
        let (num, rest) = match data.iter().position(|&b| b == b';') {
            Some(i) => (&data[..i], &data[i + 1..]),
            None => (data, &data[data.len()..]),
        };
        let Some(num) = core::str::from_utf8(num).ok().and_then(|s| s.parse::<u16>().ok()) else {
            return;
        };
        match num {
            0 | 2 => self.set_title(clean_text(rest, MAX_TITLE)),
            4 => self.osc_palette(rest),
            7 => self.set_cwd(rest),
            8 => self.osc_link(rest),
            10..=12 => self.osc_dynamic(num, rest),
            52 => self.osc_clipboard(rest),
            104 => self.osc_reset_palette(rest),
            110 => self.palette.fg = self.theme.fg,
            111 => self.palette.bg = self.theme.bg,
            112 => self.palette.cursor = self.theme.cursor,
            133 => self.osc_prompt(rest),
            _ => {}
        }
        if matches!(num, 4 | 10 | 11 | 104 | 110 | 111) {
            self.touch_all();
        }
    }

    fn set_title(&mut self, t: String) {
        self.title = t;
        self.title_gen = self.title_gen.wrapping_add(1);
    }

    pub(in crate::term) fn push_title(&mut self) {
        if self.title_stack.len() < MAX_TITLE_STACK {
            self.title_stack.push(self.title.clone());
        }
    }

    pub(in crate::term) fn pop_title(&mut self) {
        if let Some(t) = self.title_stack.pop() {
            self.set_title(t);
        }
    }
}
