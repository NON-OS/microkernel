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

//! The handle the broker keeps for a remapped vector. Bit 15 is set, so a
//! handle never equals `NO_IRTE` (0) or a VT-d index (1 to 255), and each
//! one names a device and an index in that device's table.

use spin::Mutex;

const TAG: u16 = 0x8000;
const COUNT: usize = 256;

static HANDLES: Mutex<[Option<(u16, u16)>; COUNT]> = Mutex::new([None; COUNT]);

pub fn is_amd(handle: u16) -> bool {
    handle & TAG != 0
}

/// A handle for `index` in `device`'s table, or `None` when all are out.
pub(super) fn issue(device: u16, index: u16) -> Option<u16> {
    let mut handles = HANDLES.lock();
    let slot = handles.iter().position(Option::is_none)?;
    handles[slot] = Some((device, index));
    Some(TAG | slot as u16)
}

/// The device and index behind `handle`, forgotten; `None` for a handle
/// not issued, so a second release finds nothing.
pub(super) fn take(handle: u16) -> Option<(u16, u16)> {
    let slot = (handle & !TAG) as usize;
    if !is_amd(handle) || slot >= COUNT {
        return None;
    }
    HANDLES.lock()[slot].take()
}
