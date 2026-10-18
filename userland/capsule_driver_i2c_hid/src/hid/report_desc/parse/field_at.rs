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

//! One data field of an Input or Feature item, as the walk lays it out.

use crate::hid::report_desc::Field;

/// The widest field this driver reads.
pub(super) const MAX_READ_BITS: u32 = 32;

pub(super) struct FieldAt {
    pub feature: bool,
    /// The usage of the application collection the field is in.
    pub app: u32,
    pub id: u8,
    /// Page in the top half, usage in the bottom.
    pub usage: u32,
    /// Bits from the start of the report body, and how many; the walk keeps
    /// every report within MAX_REPORT_BITS, so `at + size` fits.
    pub at: u32,
    pub size: u32,
    pub min: i64,
    pub max: i64,
    pub relative: bool,
}

impl FieldAt {
    pub fn field(&self) -> Field {
        let bound = |v: i64| v.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        Field {
            bit_offset: self.at,
            bit_size: self.size,
            logical_min: bound(self.min),
            logical_max: bound(self.max),
        }
    }
}
