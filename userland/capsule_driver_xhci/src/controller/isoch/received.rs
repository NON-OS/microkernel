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

//! What an isoch IN batch brought back: each TD's bytes from its event, the
//! batch's code, and the packets packed one after another. Pure, so the
//! host proofs hold it.

use crate::constants::{CC_MISSED_SERVICE, CC_SHORT_PACKET, CC_SUCCESS, CC_USB_TRANSACTION};

/// The bytes a TD brought in, from its event's completion code and residual
/// (the bytes not transferred). A packet the controller could not serve in
/// its interval, or lost to a bus error, brought none, and the stream goes
/// on; any other code is a failed endpoint (None).
pub fn td_bytes(cc: u8, residual: u32, td_len: u32) -> Option<u32> {
    match cc {
        CC_SUCCESS | CC_SHORT_PACKET => Some(td_len - residual.min(td_len)),
        CC_MISSED_SERVICE | CC_USB_TRANSACTION => Some(0),
        _ => None,
    }
}

/// Each TD's length into `lens`, and the batch's code: success, or missed
/// service if any packet brought nothing. Err is the code that failed it.
pub fn lengths(got: &[(u8, u32)], td_len: u32, lens: &mut [u16]) -> Result<u8, u8> {
    let mut cc = CC_SUCCESS;
    for (i, &(code, residual)) in got.iter().enumerate() {
        let bytes = td_bytes(code, residual, td_len).ok_or(code)?;
        lens[i] = bytes as u16;
        if bytes == 0 {
            cc = CC_MISSED_SERVICE;
        }
    }
    Ok(cc)
}

/// Pack each packet's bytes from its TD's slot in `half` (TDs `td_len`
/// apart) one after another into `out`; returns the bytes packed. `out`
/// holds at least the batch's TD space, so every packet fits.
pub fn pack(half: &[u8], td_len: u32, lens: &[u16], out: &mut [u8]) -> usize {
    let mut at = 0;
    for (i, &len) in lens.iter().enumerate() {
        let from = i * td_len as usize;
        let len = (len as usize).min(td_len as usize);
        out[at..at + len].copy_from_slice(&half[from..from + len]);
        at += len;
    }
    at
}
