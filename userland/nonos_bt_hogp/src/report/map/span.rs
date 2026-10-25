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

//! Which way a report goes, and the usages a main item carries as runs
//! (HID 1.11, 6.2.2.8).

/// Which way a report goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Input,
    Output,
    Feature,
}

/// A run of usages on one page, a single usage being a run of one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub page: u16,
    pub min: u16,
    pub max: u16,
}

impl Span {
    pub fn usages(&self) -> u32 {
        u32::from(self.max - self.min) + 1
    }

    /// Whether any usage of this run lies on `page` within `lo..=hi`.
    pub fn hits(&self, page: u16, lo: u16, hi: u16) -> bool {
        self.page == page && self.min <= hi && lo <= self.max
    }
}
