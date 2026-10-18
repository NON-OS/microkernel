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
use super::claim::claim;
use crate::regs::cap::ext_caps;

// Extended-capability ID for USB legacy support (xHCI 1.2 section 7.1).
const XECP_ID_LEGACY: u32 = 1;
/// USBLEGSUP and USBLEGCTLSTS, the two dwords the handoff touches.
const LEGACY_CAP_BYTES: u64 = 8;

/// Claim the controller from BIOS/SMM before it is reset. On real firmware the
/// controller is frequently still owned by SMM through USB legacy support;
/// resetting or driving it without the USBLEGSUP handshake races SMM and can
/// lose the boot keyboard or wedge the reset. Controllers without a legacy
/// capability (e.g. QEMU) advertise none and this is a no-op. The list is
/// walked only inside the `mapped_len` bytes actually mapped: Intel puts it
/// near 0x8000, past what a short mapping covers.
pub fn legacy_handoff(mmio_base: u64, mapped_len: u64) {
    let mut found = None;
    ext_caps(mmio_base, mapped_len, LEGACY_CAP_BYTES, |off, dw0| {
        if dw0 & 0xFF == XECP_ID_LEGACY {
            found = Some(off);
            return false;
        }
        true
    });
    if let Some(off) = found {
        claim(mmio_base + off);
    }
}
