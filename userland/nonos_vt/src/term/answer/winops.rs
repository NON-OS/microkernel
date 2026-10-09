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

//! XTWINOPS: only size reports and the title stack.

use crate::params::Params;
use crate::term::state::Term;

impl Term {
    /// XTWINOPS. Only size reports and the title stack: a program may not
    /// move, resize, raise or iconify the window it runs in.
    pub(in crate::term) fn window_op(&mut self, p: &Params) {
        let (w, h) = self.cell_px;
        match p.get(0) {
            14 if w > 0 => {
                let (pw, ph) = (w as usize * self.cols, h as usize * self.rows);
                self.reply_fmt(format_args!("\x1b[4;{ph};{pw}t"));
            }
            16 if w > 0 => self.reply_fmt(format_args!("\x1b[6;{h};{w}t")),
            18 => {
                let (rows, cols) = (self.rows, self.cols);
                self.reply_fmt(format_args!("\x1b[8;{rows};{cols}t"));
            }
            22 => self.push_title(),
            23 => self.pop_title(),
            _ => {}
        }
    }
}
