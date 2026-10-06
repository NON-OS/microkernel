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

//! Asking the kernel for this boot's slots.

use super::record::{parse_boot_slots, BootSlots, BOOT_SLOTS_LEN};
use crate::syscall::{call_raw, N_MK_BOOT_SLOTS};

/// Fill `out` with the `MkBootSlots` record. Needs DeviceSecret. The record's
/// length, or a negative errno: EPERM without the capability, ENOENT when the
/// kernel did not admit the loader or an image carries no trailer.
pub fn mk_boot_slots(out: &mut [u8; BOOT_SLOTS_LEN]) -> i64 {
    call_raw(N_MK_BOOT_SLOTS, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}

/// The slots, read and parsed; `None` when the kernel refused or the record
/// did not parse.
pub fn boot_slots() -> Option<BootSlots> {
    let mut buf = [0u8; BOOT_SLOTS_LEN];
    if mk_boot_slots(&mut buf) != BOOT_SLOTS_LEN as i64 {
        return None;
    }
    parse_boot_slots(&buf)
}
