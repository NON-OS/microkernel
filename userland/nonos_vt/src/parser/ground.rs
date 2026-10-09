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

//! The ground state: text, as UTF-8, and C0 controls.

use super::handler::Handler;
use super::machine::Parser;
use crate::utf8::Step;

impl Parser {
    /// A character cut short by a control byte is shown as U+FFFD, not lost
    /// silently and not merged with what follows.
    pub(super) fn flush_utf8<H: Handler>(&mut self, h: &mut H) {
        if self.utf8.pending() {
            self.utf8.reset();
            h.print(char::REPLACEMENT_CHARACTER);
        }
    }

    pub(super) fn ground<H: Handler>(&mut self, h: &mut H, b: u8) {
        if b < 0x20 {
            self.flush_utf8(h);
            h.execute(b);
            return;
        }
        if b == 0x7F {
            return;
        }
        match self.utf8.push(b) {
            Step::Pending => {}
            Step::Char(c) => h.print(c),
            Step::Invalid { again } => {
                h.print(char::REPLACEMENT_CHARACTER);
                if again {
                    self.ground(h, b);
                }
            }
        }
    }
}
