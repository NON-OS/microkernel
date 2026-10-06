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
use super::read_at::read_descriptor_at;
use crate::hid::HID_DESC_LEN;

/// HID descriptor registers in use besides the declared one: 0x0001 (ELAN and
/// most others) and 0x0020 (Synaptics). Firmware that computes its `_DSM`
/// answer at run time leaves the kernel only the 0x0001 default, so the
/// declared register is a first guess, not the only one.
pub const FALLBACK_DESC_REGS: [u16; 2] = [0x0001, 0x0020];

/// Try the exact address the firmware declared through ACPI, at the
/// declared descriptor register first and then at the registers in common
/// use. Returns the register that held the descriptor, so the driver binds
/// without probing a guessed address list.
pub fn probe_addr(
    port: u32,
    addr: u8,
    reg: u16,
    descriptor: &mut [u8; HID_DESC_LEN],
) -> Option<u16> {
    core::iter::once(reg)
        .chain(FALLBACK_DESC_REGS.into_iter().filter(|&r| r != reg))
        .find(|&r| read_descriptor_at(port, addr, r, descriptor))
}
