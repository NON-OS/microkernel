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
//! byte came from; Linux i8042_interrupt routes every byte by that bit. A
//! key held at boot, or a touchpad the firmware left reporting, puts bytes
//! in the buffer while the other port's reply is awaited, and a keystroke
//! taken for the mouse's ACK once turned the aux port off for the session.
//! A byte from the other port goes where the caller says: during bring-up,
//! before anything serves the keyboard, it is dropped and counted; when the
//! serve loop brings a mouse up again, a key goes to the keyboard's decode.

use nonos_libc::{mk_pio_read, Deadline};

use super::dropped::drop_byte;
use super::pause::Pause;
use super::wait::WaitError;
use crate::constants::{DATA_OFFSET, STATUS_AUX_DATA, STATUS_OFFSET, STATUS_OUTPUT_FULL};

/// The next byte from the aux port (`aux` true) or from the keyboard and the
/// controller (`aux` false) within `timeout_ms`; None when none came. A byte
/// from the other port is dropped.
pub fn read_port(grant_id: u64, aux: bool, timeout_ms: u64) -> Result<Option<u8>, WaitError> {
    read_routed(grant_id, aux, timeout_ms, &mut drop_byte)
}

/// As `read_port`, handing each byte from the other port to `other`.
pub fn read_routed(
    grant_id: u64,
    aux: bool,
    timeout_ms: u64,
    other: &mut dyn FnMut(u8),
) -> Result<Option<u8>, WaitError> {
    let deadline = Deadline::after_ms(timeout_ms);
    let mut pause = Pause::new();
    loop {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err(WaitError::Read);
        }
        if status as u8 & STATUS_OUTPUT_FULL != 0 {
            let mut data = 0u32;
            if mk_pio_read(grant_id, DATA_OFFSET, 1, &mut data) < 0 {
                return Err(WaitError::Read);
            }
            if (status as u8 & STATUS_AUX_DATA != 0) == aux {
                return Ok(Some(data as u8));
            }
            other(data as u8);
        }
        // Checked after the other port's byte too: a port that streams
        // without pause must not hold the bring-up past its bound.
        if deadline.expired() {
            return Ok(None);
        }
        pause.wait();
    }
}
