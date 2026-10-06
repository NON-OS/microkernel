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
use super::read_port;
use super::wait::{wait_input_clear, WaitError, ACK_TIMEOUT_MS, CTL_TIMEOUT_MS};
use crate::constants::{DATA_OFFSET, KBD_ENABLE_SCANNING, MOUSE_ACK};
use nonos_libc::mk_pio_write;

/// Send the keyboard a byte and wait for its answer. Ok(Some(byte)) is what
/// came back, Ok(None) is a keyboard that stayed silent past the bound.
pub(super) fn send(grant_id: u64, byte: u8, timeout_ms: u64) -> Result<Option<u8>, &'static str> {
    match wait_input_clear(grant_id, CTL_TIMEOUT_MS) {
        Ok(()) => {}
        Err(WaitError::Read) => return Err("kbd status read failed"),
        Err(WaitError::Timeout) => return Err("kbd input buffer busy"),
    }
    if mk_pio_write(grant_id, DATA_OFFSET, 1, byte as u32) < 0 {
        return Err("kbd write failed");
    }
    read_reply(grant_id, timeout_ms)
}

/// The next byte the keyboard sends within `timeout_ms`, if any; a byte
/// from the aux port is not the keyboard's.
pub(super) fn read_reply(grant_id: u64, timeout_ms: u64) -> Result<Option<u8>, &'static str> {
    match read_port(grant_id, false, timeout_ms) {
        Ok(byte) => Ok(byte),
        Err(WaitError::Read) => Err("kbd status read failed"),
        Err(WaitError::Timeout) => Ok(None),
    }
}

/// Start scanning (0xF4) and eat the acknowledgement, so a late ACK is never
/// read as the reply to a later controller command. True when it was ACKed.
pub fn enable_scanning(grant_id: u64) -> Result<bool, &'static str> {
    Ok(send(grant_id, KBD_ENABLE_SCANNING, ACK_TIMEOUT_MS)? == Some(MOUSE_ACK))
}
