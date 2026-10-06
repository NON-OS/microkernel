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
    /// Bytes a program or the shell wrote, through output processing.
    pub fn feed_raw(&mut self, bytes: &[u8]) {
        if !self.onlcr {
            self.vt.feed(bytes);
            return;
        }
        let mut rest = bytes;
        while let Some(i) = rest.iter().position(|&b| b == b'\n') {
            self.vt.feed(&rest[..i]);
            self.vt.feed(b"\r\n");
            rest = &rest[i + 1..];
        }
        self.vt.feed(rest);
    }
}
