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

//! Questions the bind and teardown paths ask of the grant records.

extern crate alloc;

use alloc::vec::Vec;

use super::super::grant::IrqGrantKind;
use super::store::RECORDS;

// True if the device already has an MSI or MSI-X grant. MSI-X binds stay
// all-or-nothing per device, and a function never gets both kinds enabled.
pub(in super::super) fn has_message_grant(device_id: u64) -> bool {
    RECORDS.lock().iter().any(|g| g.device_id == device_id && g.kind != IrqGrantKind::Intx)
}

pub(in super::super) fn count_msix_for_device(device_id: u64) -> usize {
    RECORDS
        .lock()
        .iter()
        .filter(|g| g.device_id == device_id && g.kind == IrqGrantKind::Msix)
        .count()
}

pub(in super::super) fn vectors_for_pid(pid: u32) -> Vec<u8> {
    RECORDS.lock().iter().filter(|g| g.pid == pid).map(|g| g.vector).collect()
}
