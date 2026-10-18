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

//! The global items in force (HID 1.11, 6.2.2.7): the usage page, logical
//! range, report size, count and ID that each main item takes its fields by.

use super::limits::{Refused, MAX_REPORT_BITS};

#[derive(Clone, Copy, Debug, Default)]
pub struct Globals {
    pub page: u16,
    pub min: i64,
    pub max: i64,
    pub size: u32,
    pub count: u32,
    /// 0 until a Report ID item names one.
    pub id: u8,
}

impl Globals {
    /// The bits one main item takes: report size times count, bounded.
    pub fn bits(&self) -> Result<u32, Refused> {
        self.size.checked_mul(self.count).filter(|&b| b <= MAX_REPORT_BITS).ok_or(Refused)
    }
}
