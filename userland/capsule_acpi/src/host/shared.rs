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

//! Memory firmware shares with the capsule outside any OperationRegion
//! access, such as the UCSI mailbox a _CRS names: mapped through the same
//! grants, read and written once at the width asked.

use super::memory_io;
use super::state::CapsuleHost;

pub fn read_shared(
    host: &mut CapsuleHost,
    addr: u64,
    width: u32,
    region: (u64, u64),
) -> Option<u64> {
    let va = host.windows.va(addr, u64::from(width), region)?;
    memory_io::read(va, width)
}

pub fn write_shared(
    host: &mut CapsuleHost,
    addr: u64,
    width: u32,
    region: (u64, u64),
    v: u64,
) -> bool {
    match host.windows.va(addr, u64::from(width), region) {
        Some(va) => memory_io::write(va, width, v),
        None => false,
    }
}
