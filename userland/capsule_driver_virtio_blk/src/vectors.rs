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

//! Legacy virtio with MSI-X enabled: which vector each source raises.
//!
//! With MSI-X on, the device resets both vectors to NO_VECTOR and raises
//! nothing at all, so every request was found only when its wait timed out,
//! 100 ms later. The request queue is pointed at table entry 0, the one the
//! kernel bound, and configuration changes at no vector.

use nonos_libc::mk_debug;

use super::regs::Regs;

const LEG_MSIX_CONFIG_VECTOR: usize = 0x14;
const LEG_MSIX_QUEUE_VECTOR: usize = 0x16;
const NO_VECTOR: u16 = 0xFFFF;

/// Called with the request queue selected.
pub unsafe fn assign(regs: Regs) {
    regs.w16(LEG_MSIX_CONFIG_VECTOR, NO_VECTOR);
    regs.w16(LEG_MSIX_QUEUE_VECTOR, 0);
    if regs.r16(LEG_MSIX_QUEUE_VECTOR) != 0 {
        let line = b"[BLK] device refused MSI-X entry 0; completions are found by timeout\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}
