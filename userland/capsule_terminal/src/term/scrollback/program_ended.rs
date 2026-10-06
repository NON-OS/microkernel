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

use super::types::Scrollback;

impl Scrollback {
    /// A program ended. Whatever modes it left on are turned off, as `reset`
    /// would: a full-screen program that crashed must not leave the shell on
    /// its alternate screen, hide the cursor, or keep sending mouse reports
    /// into the prompt. Output that stopped mid-line gets its line ended, so
    /// the next command's output starts at the left.
    pub fn program_ended(&mut self) {
        self.vt.feed(
            b"\x1b[?1049l\x1b[?1l\x1b[?2004l\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l\
              \x1b[?1004l\x1b[?25h\x1b[0m\x1b(B",
        );
        if self.vt.cursor().x > 0 {
            self.vt.feed(b"\r\n");
        }
        self.onlcr = true;
    }
}
