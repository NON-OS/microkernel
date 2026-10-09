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

//! One reply byte from the port that owes it. The keyboard and the aux port
//! share one output buffer, and status bit 5 (AUXDATA) says which of them a
//! byte came from; Linux i8042_interrupt routes every byte by that bit. On a
//! real laptop a key held at boot, or a touchpad the firmware left
//! reporting, puts bytes in the buffer while the other port's reply is
//! awaited. Taken for the reply, a keystroke read as the mouse's ACK made
//! the bring-up turn the aux port off for the whole session. A byte from the
//! other port is dropped here: one keystroke typed during bring-up is lost
//! instead of the touchpad.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_pio_read, Deadline};

use super::wait::WaitError;
use crate::constants::{DATA_OFFSET, STATUS_AUX_DATA, STATUS_OFFSET, STATUS_OUTPUT_FULL};

/// Bytes from the other port dropped so far, for the bring-up's log line.
static DROPPED: AtomicU32 = AtomicU32::new(0);

pub fn dropped() -> u32 {
    DROPPED.load(Ordering::Relaxed)
}

/// The next byte from the aux port (`aux` true) or from the keyboard and the
/// controller (`aux` false) within `timeout_ms`; None when none came.
pub fn read_port(grant_id: u64, aux: bool, timeout_ms: u64) -> Result<Option<u8>, WaitError> {
    let deadline = Deadline::after_ms(timeout_ms);
    loop {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err(WaitError::Read);
        }
        let status = status as u8;
        if status & STATUS_OUTPUT_FULL != 0 {
            let mut data = 0u32;
            if mk_pio_read(grant_id, DATA_OFFSET, 1, &mut data) < 0 {
                return Err(WaitError::Read);
            }
            if (status & STATUS_AUX_DATA != 0) == aux {
                return Ok(Some(data as u8));
            }
            DROPPED.fetch_add(1, Ordering::Relaxed);
        }
        // Checked after a dropped byte too: a port that streams without
        // pause must not hold the bring-up past its bound.
        if deadline.expired() {
            return Ok(None);
        }
        core::hint::spin_loop();
    }
}
