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
//! A controller's window base, read back once the caller holds it.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

use super::defs::MAX_DEVICES;

/// The base of bar 0 in the record of `device_id`, found by that id and
/// never by its place in the list, read after the caller's claim: the
/// device list shows a window's base only to the device's holder, and 0 to
/// anyone else. None when the list fails, holds no record of that id, or
/// that record's bar 0 is no longer an MMIO window with a size.
pub fn claimed_base(device_id: u64) -> Option<u64> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let listed = usize::try_from(n).ok()?;
    let own = buf.iter().take(listed).find(|r| r.device_id == device_id)?;
    let bar = own.bars[0];
    (bar.kind == BAR_KIND_MMIO && bar.size != 0).then_some(bar.base)
}
