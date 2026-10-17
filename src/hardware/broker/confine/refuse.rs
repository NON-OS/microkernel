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

extern crate alloc;

use alloc::vec::Vec;

use super::attach::Refused;
use super::table::{say, Capsule};
use crate::memory::iommu::DeviceAddress;

/// Say why, and drop the capsule's entry if this was to be its first device.
pub(super) fn refuse(
    all: &mut Vec<Capsule>,
    pos: usize,
    why: &[u8],
    pid: u32,
    address: DeviceAddress,
) -> Result<(), Refused> {
    say(why, pid, address);
    if all[pos].devices.is_empty() {
        all.remove(pos);
    }
    Err(Refused)
}
