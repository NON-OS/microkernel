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

use super::describe;
use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::{Fdt, FdtError};

/// As many `/reserved-memory` ranges as one pass reads.
const FIRMWARE_RANGES: usize = 8;

/// Everything in RAM the allocator must leave alone: the blob itself (the
/// platform bus reads it again later), the initrd, the reservation block and
/// the fixed `/reserved-memory` ranges, such as a Pi's spin tables.
pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) -> Result<(), &'static [u8]> {
    info.reserved.add(info.dtb_base, info.dtb_size)?;
    if let Some((start, end)) = fdt.initrd().map_err(describe)? {
        info.initrd = Some((start, end));
        info.reserved.add(start, end - start)?;
    }
    for entry in fdt.reservations() {
        let (base, size) = entry.map_err(describe)?;
        info.reserved.add(base, size)?;
    }
    let mut firmware = [(0u64, 0u64); FIRMWARE_RANGES];
    let count = fdt.reserved_memory(&mut firmware).map_err(|e| match e {
        FdtError::TooLarge => &b"more /reserved-memory ranges than one pass reads"[..],
        other => describe(other),
    })?;
    for &(base, size) in &firmware[..count] {
        info.reserved.add(base, size)?;
    }
    Ok(())
}
