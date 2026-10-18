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

//! A setup frame: 192 bytes, sixteen addresses for the perfect filter, each
//! as three sixteen-bit halves in the low half of a longword.

use super::events::Event;
use super::part::Part;

impl Part {
    pub(super) fn take_setup(&mut self, buf: u32, len: usize) {
        assert_eq!(len, 192, "a perfect-filter setup frame is 192 bytes");
        let mut b = [0u8; 192];
        self.mem.read(buf, &mut b);
        for (e, entry) in self.filter.iter_mut().enumerate() {
            for half in 0..3 {
                entry[2 * half] = b[e * 12 + half * 4];
                entry[2 * half + 1] = b[e * 12 + half * 4 + 1];
            }
        }
        self.log.push(Event::Setup);
    }

    /// Whether the filter passes a frame to `dest`: broadcast always, else
    /// an address the setup frame loaded.
    pub fn passes(&self, dest: &[u8]) -> bool {
        dest == [0xFF; 6] || self.filter.iter().any(|a| a[..] == *dest)
    }
}
