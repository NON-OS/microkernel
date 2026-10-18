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

//! The keyboard's lock LEDs, set from the serve loop (never from inside a
//! drain) with the same bounded wait every bring-up step has. The LEDs are
//! the driver's own lock state: set once at start, whatever firmware left
//! lit, and again after every lock key, so the light and the case agree.

use nonos_libc::{mk_pio_write, Deadline};

use super::pause::Pause;
use super::wait::{wait_input_clear, ACK_TIMEOUT_MS, CTL_TIMEOUT_MS};
use crate::constants::{DATA_OFFSET, KBD_SET_LEDS, MOUSE_ACK};

/// Set the LEDs to `bits` (bit 0 Scroll, 1 Num, 2 Caps): 0xED, then the
/// bitmap, each acknowledged within ACK_TIMEOUT_MS. `drain` empties the
/// output buffer the usual way, so a keystroke or a mouse byte that lands
/// meanwhile is delivered as always, and answers the keyboard's ACK or
/// RESEND if one came. False when a byte was refused or never answered;
/// the caller leaves the LEDs until the next lock key rather than retry.
pub fn set_leds(grant_id: u64, bits: u8, mut drain: impl FnMut() -> Option<u8>) -> bool {
    send(grant_id, KBD_SET_LEDS, &mut drain) && send(grant_id, bits & 0x07, &mut drain)
}

fn send(grant_id: u64, byte: u8, drain: &mut impl FnMut() -> Option<u8>) -> bool {
    if wait_input_clear(grant_id, CTL_TIMEOUT_MS).is_err()
        || mk_pio_write(grant_id, DATA_OFFSET, 1, u32::from(byte)) < 0
    {
        return false;
    }
    let deadline = Deadline::after_ms(ACK_TIMEOUT_MS);
    let mut pause = Pause::new();
    loop {
        if let Some(reply) = drain() {
            return reply == MOUSE_ACK;
        }
        if deadline.expired() {
            return false;
        }
        pause.wait();
    }
}
