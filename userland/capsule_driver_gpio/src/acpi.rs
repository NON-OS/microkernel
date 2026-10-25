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

//! What the GPIO capsule asks of capsule_acpi (port 4530), raw as its
//! protocol is: an op byte and arguments; a status word and a body back.
//! SERIAL (9) gives each serial-bus device's GpioInt pin; HOLDER (21)
//! answers whether a pid holds a PCI function, through the kernel's claim
//! test, so a pin's events go only to its device's holder. `aei` asks
//! GPIO_EVENT (13).

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;

const ACPI_PORT: u64 = 4530;
const WAIT_MS: u64 = 2_000;
const OP_SERIAL: u8 = 9;
const OP_HOLDER: u8 = 21;

pub fn call(req: &[u8]) -> Result<Vec<u8>, i32> {
    let mut resp = vec![0u8; 512];
    let n = mk_ipc_call_timeout(
        ACPI_PORT,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        WAIT_MS,
    );
    if n < 4 {
        return Err(if n < 0 { n as i32 } else { -5 });
    }
    let status = i32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]);
    resp.truncate(n as usize);
    if status != 0 {
        return Err(status);
    }
    Ok(resp.split_off(4))
}

pub fn serial(index: u32) -> Result<Vec<u8>, i32> {
    let mut req = vec![OP_SERIAL];
    req.extend_from_slice(&index.to_le_bytes());
    call(&req)
}

/// Whether `pid` holds the PCI function `key` (segment << 16 | bus << 8
/// | devfn).
pub fn holds(pid: u32, key: u64) -> bool {
    let mut req = vec![OP_HOLDER];
    req.extend_from_slice(&pid.to_le_bytes());
    req.extend_from_slice(&key.to_le_bytes());
    call(&req).is_ok_and(|b| b.first() == Some(&1))
}
