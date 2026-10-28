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

//! Into the firmware sent: Intel Reset (0xfc01) as a soft reset, patches
//! enabled, DDC not reloaded, booting from the address the firmware named
//! (`btintel_send_intel_reset`). Nothing answers it (btusb injects the
//! Command Complete); the firmware says it runs with its bootup event
//! (0xff, 0x02, six octets), within a second or not at all (`btintel_boot`).

use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode::Opcode;

pub const INTEL_RESET: u16 = 0xfc01;
const BOOT_LOOKS: u32 = 1_000;

pub fn boot<P: UsbPipes>(w: &mut Wire<P>, addr: u32) -> Result<(), HciError> {
    let [a, b, c, d] = addr.to_le_bytes();
    w.post(INTEL_RESET, &[0x00, 0x01, 0x00, 0x01, a, b, c, d])?;
    match w.wait(BOOT_LOOKS, |e| matches!(e, [0xff, 7, 0x02, ..]).then_some(()))? {
        Some(()) => Ok(()),
        None => {
            to_bootloader(w);
            Err(HciError::Timeout(Opcode(INTEL_RESET)))
        }
    }
}

/// A hard reset back into the bootloader, patches and DDC reloaded, from
/// the current image: after a download that failed, or for firmware that
/// is not the carried build. The device leaves the bus and comes back,
/// and is found again (`btintel_reset_to_bootloader`).
pub fn to_bootloader<P: UsbPipes>(w: &mut Wire<P>) {
    let _ = w.post(INTEL_RESET, &[0x01, 0x01, 0x01, 0x00, 0, 0, 0, 0]);
}
