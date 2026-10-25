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

//! Before soft-off, `capsule_acpi` runs the firmware's \_TTS(5) and
//! \_PTS(5) and gives the kernel the SLP_TYP values \_S5 evaluates to
//! (ACPI 6.5, 16.1), so the PM1 write is the one firmware asked for, even
//! where \_S5 is a method the kernel's constant reader cannot read. When
//! the capsule is not running, the call times out and the kernel's own
//! reading stands.

use nonos_libc::mk_ipc_call_timeout;

const ACPI_PORT: u64 = 4530;
const OP_PREPARE_SLEEP: u8 = 5;
const SOFT_OFF: u8 = 5;
const WAIT_MS: u64 = 3_000;
const EIO: i32 = -5;

/// The capsule's status, or the call's error: 0 when firmware is ready.
pub fn prepare_soft_off() -> i32 {
    let req = [OP_PREPARE_SLEEP, SOFT_OFF];
    let mut resp = [0u8; 16];
    let n = mk_ipc_call_timeout(
        ACPI_PORT,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        WAIT_MS,
    );
    match n {
        n if n < 0 => n as i32,
        n if n < 4 => EIO,
        _ => i32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]),
    }
}
