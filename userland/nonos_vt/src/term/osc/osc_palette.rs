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

//! Setting and reporting palette colours: OSC 4, 10 to 12 and 104.

use super::osc_color::parse_spec;
use crate::term::state::Term;

impl Term {
    pub(in crate::term) fn reply_colour(&mut self, prefix: &str, c: u32) {
        // Each channel as xterm reports it: sixteen bits, the byte repeated.
        let ch = |shift: u32| ((c >> shift) & 0xFF) * 257;
        let (r, g, b) = (ch(16), ch(8), ch(0));
        self.reply_fmt(format_args!("\x1b]{prefix};rgb:{r:04x}/{g:04x}/{b:04x}\x1b\\"));
    }

    /// `4;index;spec[;index;spec...]`.
    pub(in crate::term) fn osc_palette(&mut self, rest: &[u8]) {
        let mut parts = rest.split(|&b| b == b';');
        while let (Some(i), Some(spec)) = (parts.next(), parts.next()) {
            let Some(i) = core::str::from_utf8(i).ok().and_then(|s| s.parse::<u8>().ok()) else {
                return;
            };
            if spec == b"?" {
                let c = self.palette.colors[i as usize];
                self.reply_colour(&alloc::format!("4;{i}"), c);
            } else if let Some(c) = parse_spec(spec) {
                self.palette.colors[i as usize] = c;
            }
        }
    }

    /// 10 foreground, 11 background, 12 cursor.
    pub(in crate::term) fn osc_dynamic(&mut self, num: u16, spec: &[u8]) {
        let slot = match num {
            10 => &mut self.palette.fg,
            11 => &mut self.palette.bg,
            _ => &mut self.palette.cursor,
        };
        if spec == b"?" {
            let c = *slot;
            self.reply_colour(&alloc::format!("{num}"), c);
        } else if let Some(c) = parse_spec(spec) {
            *slot = c;
        }
    }

    /// 104 with no indices resets the whole palette to the theme.
    pub(in crate::term) fn osc_reset_palette(&mut self, rest: &[u8]) {
        if rest.is_empty() {
            self.palette.colors = self.theme.colors;
            return;
        }
        for i in rest.split(|&b| b == b';') {
            if let Some(i) = core::str::from_utf8(i).ok().and_then(|s| s.parse::<u8>().ok()) {
                self.palette.colors[i as usize] = self.theme.colors[i as usize];
            }
        }
    }
}
