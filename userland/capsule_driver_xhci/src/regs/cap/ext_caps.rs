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
use super::xecp::walk_ext_caps;
use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

/// The same walk over the mapped registers at `mmio_base`.
pub fn ext_caps(mmio_base: u64, mapped_len: u64, need: u64, visit: impl FnMut(u64, u32) -> bool) {
    // SAFETY: `mmio_base` starts the mapped window, and the capability registers are its first 32
    // bytes.
    let hccparams1 = unsafe { mmio_read32(mmio_base + HCCPARAMS1) };
    // SAFETY: `walk_ext_caps` reads only offsets whose dwords it checked lie
    // inside the mapped window.
    let read = |off| unsafe { mmio_read32(mmio_base + off) };
    walk_ext_caps(hccparams1, mapped_len, need, read, visit);
}
