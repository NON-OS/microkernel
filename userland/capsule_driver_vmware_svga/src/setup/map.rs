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

//! Mapping part of a memory BAR. The broker maps whole pages around the
//! request and hands back the address of its first byte.

use nonos_libc::{mk_mmio_map, MmioMapOut};

pub fn map(
    device_id: u64,
    epoch: u64,
    bar: u32,
    offset: u64,
    len: u64,
) -> Result<MmioMapOut, &'static str> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    if mk_mmio_map(device_id, epoch, bar, 0, offset, len, &mut out) < 0 || out.length < len {
        return Err("mmio map failed");
    }
    Ok(out)
}
