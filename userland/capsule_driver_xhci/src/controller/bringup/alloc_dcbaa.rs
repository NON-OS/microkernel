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
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
const DCBAA_ENTRY_BYTES: u64 = 8;
/// The Device Context Base Address Array for `max_slots` slots, entry 0
/// naming the scratchpad array (xHCI 1.2 section 6.1). Only memory: the
/// controller is told of it by `program_dcbaa`.
pub fn alloc_dcbaa(
    pool: &DmaPool,
    max_slots: u8,
    scratchpad_array_phys: u64,
) -> XhciResult<DmaRegion> {
    let bytes = ((max_slots as u64) + 1) * DCBAA_ENTRY_BYTES;
    let region = pool.alloc(bytes)?;
    region.zero();
    if scratchpad_array_phys != 0 {
        // Entry 0: the region was just mapped with room for it.
        let _ = region.write_u64(0, scratchpad_array_phys);
    }
    Ok(region)
}
