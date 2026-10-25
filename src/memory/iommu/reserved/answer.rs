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

//! What the hook answers for one device.

use super::range::ReservedRange;

/// Ranges kept for one device: a firmware table naming more refuses the
/// claim rather than leaving one out.
pub const MAX_RESERVED: usize = 8;

/// The ranges `device_reserved_regions` found for a device, or why they
/// cannot be honoured, in which case the claim is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reserved {
    pub ranges: [ReservedRange; MAX_RESERVED],
    pub count: usize,
    /// Why the firmware's description of this device's ranges could not be
    /// read as one; said once, and the device is not attached.
    pub unresolved: Option<&'static [u8]>,
}

impl Reserved {
    pub const NONE: Self = Self {
        ranges: [ReservedRange { iova: 0, phys: 0, len: 0 }; MAX_RESERVED],
        count: 0,
        unresolved: None,
    };

    pub fn ranges(&self) -> &[ReservedRange] {
        &self.ranges[..self.count.min(MAX_RESERVED)]
    }

    /// Add `range`; one past `MAX_RESERVED` makes the answer unresolved.
    pub fn push(&mut self, range: ReservedRange) {
        match self.ranges.get_mut(self.count) {
            Some(slot) => {
                *slot = range;
                self.count += 1;
            }
            None => self.unresolved = Some(b"more reserved ranges than kept"),
        }
    }
}
