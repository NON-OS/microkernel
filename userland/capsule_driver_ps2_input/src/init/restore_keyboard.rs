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

//! The mouse bring-up holds the keyboard port off (0xAD) while it reads the
//! configuration byte, and only its configuration write turns the port back
//! on. When the aux bring-up stops before that write, the keyboard has to
//! be turned back on here, or no key works until the next boot. 0xAE also
//! clears bit 4 of the configuration byte, so this is harmless when the
//! write did happen.

use super::wait::{wait_input_clear, CTL_TIMEOUT_MS};
use crate::constants::{CTL_ENABLE_KBD, STATUS_OFFSET};
use nonos_libc::{mk_debug, mk_pio_write};

/// Send 0xAE and say on the console whether the keyboard port is back on.
pub fn restore_keyboard(grant_id: u64) {
    let sent = wait_input_clear(grant_id, CTL_TIMEOUT_MS).is_ok()
        && mk_pio_write(grant_id, STATUS_OFFSET, 1, CTL_ENABLE_KBD as u32) >= 0;
    let line: &[u8] = if sent {
        b"[driver_ps2] keyboard port turned back on after the aux bring-up stopped\n"
    } else {
        b"[driver_ps2] keyboard port could not be turned back on: controller busy\n"
    };
    let _ = mk_debug(line.as_ptr(), line.len());
}
