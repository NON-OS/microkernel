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

//! The unit's register window as the device tree describes it, for the
//! broker's forbidden list: a capsule mapping these pages could turn
//! translation off or point a stream at any memory. Described is enough,
//! in service or not: a unit that failed bring-up is no capsule's either.

use super::state::DESCRIBED;

/// Register pages 0 and 1, 64 KiB each (Arm SMMUv3, the register layout;
/// recalled, unverified): the least a described window is taken to cover.
const MIN_WINDOW: u64 = 0x2_0000;

/// Base and length of the described unit's registers, or `None` when the
/// tree names no SMMUv3.
pub fn described_window() -> Option<(u64, u64)> {
    let (info, _) = DESCRIBED.get()?;
    Some((info.base, info.size.max(MIN_WINDOW)))
}
