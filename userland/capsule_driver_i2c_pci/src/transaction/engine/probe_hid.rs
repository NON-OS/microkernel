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
use nonos_libc::mk_idle_ms;

use crate::driver::Driver;
use crate::transaction::{TransferError, TransferRequest, FLAG_RESTART_ON_READ};

use super::presence::Presence;
use super::transfer::transfer;

/// HID descriptor registers in use: the one the firmware declared first,
/// then 0x0001 (ELAN and most others) and 0x0020 (Synaptics), which is what
/// is left when the firmware computes the `_DSM` answer at run time.
fn descriptor_registers(declared: u16) -> [u16; 3] {
    match declared {
        0x0001 => [0x0001, 0x0020, 0x0001],
        0x0020 => [0x0020, 0x0001, 0x0020],
        other => [other, 0x0001, 0x0020],
    }
}

const PROBE_ATTEMPTS: u32 = 4;
const PROBE_RETRY_MS: u64 = 10;

// Confirm an i2c-HID device: address its HID descriptor register and read the
// descriptor's head back. ELAN and other parts NAK a bare read, so a
// register-addressed read is what detects them. The first two fields tell a
// real HID descriptor (length 30, version 1.00) from any other device that
// happens to answer at the address on this bus.
pub fn probe_hid(driver: &Driver, addr: u8, declared_reg: u16) -> Result<Presence, TransferError> {
    let mut acked = false;
    // A touchpad can be slow to answer right after its controller leaves
    // reset or while it wakes from its own sleep, so one miss is not absence.
    for attempt in 0..PROBE_ATTEMPTS {
        if attempt > 0 {
            let _ = mk_idle_ms(PROBE_RETRY_MS);
        }
        for reg in descriptor_registers(declared_reg) {
            let r = reg.to_le_bytes();
            let req = TransferRequest { addr, flags: FLAG_RESTART_ON_READ, write: &r, read_len: 4 };
            match transfer(driver, req) {
                Ok(res) if res.read_len >= 4 => {
                    acked = true;
                    let len = u16::from_le_bytes([res.read[0], res.read[1]]);
                    let version = u16::from_le_bytes([res.read[2], res.read[3]]);
                    if len == 30 && version == 0x0100 {
                        return Ok(Presence::HidDescriptor(reg));
                    }
                }
                Ok(_) => acked = true,
                Err(TransferError::Nack) => {}
                Err(e) => return Err(e),
            }
        }
    }
    Ok(if acked { Presence::Acked } else { Presence::Absent })
}
