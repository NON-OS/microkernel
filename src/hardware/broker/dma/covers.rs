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

//! Whether live grants on a device cover a page named to it, as the bus
//! sees it: the caller's own, or anyone's.

use super::records::RECORDS;

const PAGE: u64 = 4096;

pub fn covers(pid: u32, device_id: u64, device_addr: u64) -> bool {
    let Some(end) = device_addr.checked_add(PAGE) else { return false };
    RECORDS.lock().iter().any(|g| {
        g.pid == pid
            && g.device_id == device_id
            && g.device_addr <= device_addr
            && g.device_addr.checked_add(g.length).is_some_and(|e| end <= e)
    })
}

/// Whether any live grant on a device covers a page, whoever holds it: a
/// VMBus channel's frames are its driver's, named to the host by the core.
pub fn covers_any(device_id: u64, device_addr: u64) -> bool {
    let Some(end) = device_addr.checked_add(PAGE) else { return false };
    RECORDS.lock().iter().any(|g| {
        g.device_id == device_id
            && g.device_addr <= device_addr
            && g.device_addr.checked_add(g.length).is_some_and(|e| end <= e)
    })
}
