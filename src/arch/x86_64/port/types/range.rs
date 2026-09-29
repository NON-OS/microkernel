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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortRange {
    start: u16,
    count: u16,
}

impl PortRange {
    pub const fn new(start: u16, count: u16) -> Self {
        Self { start, count }
    }

    pub const fn start(&self) -> u16 {
        self.start
    }

    pub const fn count(&self) -> u16 {
        self.count
    }

    pub const fn end(&self) -> u16 {
        self.start.saturating_add(self.count)
    }

    /* Offset from start, not a comparison with end(): end() saturates at
    0xFFFF, so a range reaching the top would never hold port 0xFFFF. */
    pub const fn contains(&self, port: u16) -> bool {
        port >= self.start && port - self.start < self.count
    }

    pub const fn overlaps(&self, other: &PortRange) -> bool {
        let (a, b) = (self.start as u32, other.start as u32);
        a < b + other.count as u32 && b < a + self.count as u32
    }
}
