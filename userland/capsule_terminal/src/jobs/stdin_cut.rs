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

//! Where a queued input message is cut, so no message outgrows one read
//! and no UTF-8 character is split between two messages.

/// The largest message: what the reading side takes in one read.
pub const MESSAGE_MAX: usize = 4096;

/// Where the next message ends: the whole rest, or `MESSAGE_MAX` moved back
/// off at most three UTF-8 continuation bytes, so it always moves forward.
pub fn cut_at(rest: &[u8]) -> usize {
    let mut cut = rest.len().min(MESSAGE_MAX);
    while cut < rest.len() && cut > MESSAGE_MAX - 3 && rest[cut] & 0xC0 == 0x80 {
        cut -= 1;
    }
    cut
}
