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

//! Each platform device's namespace path and identity, kept by the kernel
//! beside its record: the wire record a capsule sees has no room for a
//! path, and the census and the logs name the device by it.

use alloc::vec::Vec;
use spin::Mutex;

pub struct Named {
    pub device_id: u64,
    pub hid: [u8; 8],
    pub uid: u64,
    pub path: Vec<u8>,
}

static NAMES: Mutex<Vec<Named>> = Mutex::new(Vec::new());

pub fn keep(named: Named) {
    NAMES.lock().push(named);
}

/// The path of a platform device the capsule described.
pub fn path_of(device_id: u64) -> Option<Vec<u8>> {
    NAMES.lock().iter().find(|n| n.device_id == device_id).map(|n| n.path.clone())
}
