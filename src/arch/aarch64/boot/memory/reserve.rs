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

use crate::arch::aarch64::boot::info::BootInfo;

/// Take what the device tree reserved out of the allocator's hands. Runs once
/// the allocator exists and before anything has asked it for a frame; ranges
/// outside the managed span are ignored there, being nobody's to hand out.
pub(super) fn apply(info: &BootInfo) {
    for &(start, end) in info.reserved.ranges() {
        crate::memory::phys::reserve(start, end);
    }
}
