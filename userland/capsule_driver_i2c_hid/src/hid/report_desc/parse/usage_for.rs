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

// The nth field's usage; when a report declares more fields than usages, the
// last usage applies to the remainder (per the HID spec).
pub(super) fn usage_for(usages: &[u16], n: usize, index: usize) -> u16 {
    if n == 0 {
        0
    } else if index < n {
        usages[index]
    } else {
        usages[n - 1]
    }
}

pub(super) const MAX_USAGES: usize = 64;

/// The Usage items declared since the last main item, in order.
#[derive(Clone, Copy)]
pub(super) struct Usages {
    pub list: [u16; MAX_USAGES],
    pub n: usize,
}

impl Default for Usages {
    fn default() -> Self {
        Self { list: [0; MAX_USAGES], n: 0 }
    }
}
