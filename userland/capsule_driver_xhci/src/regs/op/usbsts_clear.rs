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
use crate::constants::USBSTS;
use crate::regs::mmio_write32;

/// Acknowledge the USBSTS bits in `w1c_mask` and no others. Every writable
/// USBSTS bit is RW1C (HSE, EINT, PCD, SRE), so writing back what was read
/// would acknowledge a port change or an interrupt nobody has looked at.
pub fn usbsts_clear(op_base: u64, w1c_mask: u32) {
    mmio_write32(op_base + USBSTS, w1c_mask);
}
