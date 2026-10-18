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

use super::stage::translate_stage1_read;

const DIRECT_START: u64 = 0xFFFF_8000_0000_0000;
/// The direct map's one top-level slot, 512 GiB.
const DIRECT_END: u64 = DIRECT_START + (1 << 39);

/// The physical address behind kernel virtual `virt`. Inside the direct
/// map it is arithmetic; anywhere else (the heap, mapped windows, the rest
/// of the high half) only the MMU knows, so it is asked (AT S1E1R).
pub fn virt_to_phys(virt: u64) -> Option<u64> {
    if (DIRECT_START..DIRECT_END).contains(&virt) {
        return Some(virt - DIRECT_START);
    }
    translate_stage1_read(virt).ok()
}
