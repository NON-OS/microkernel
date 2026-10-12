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

//! OP_TX_PACKET: one frame to send, from a whole Ethernet header up to a
//! maximum frame. Answered as soon as the frame is queued.

use crate::constants::{ETH_HEADER_LEN, MAX_ETHERNET_FRAME};
use crate::protocol::{Request, E_AGAIN, E_INVAL, E_IO, E_MSGSIZE};
use crate::queue::TxError;
use crate::server::error::reply_with_status;
use crate::setup::Driver;
use crate::tx::send;

/// True when the device has stalled and must be reset.
pub fn handle(
    sender_pid: u32,
    driver: &mut Driver,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> bool {
    let (status, stalled) = if req.payload_len as usize != body.len() {
        (E_MSGSIZE, false)
    } else if body.len() < ETH_HEADER_LEN || body.len() > MAX_ETHERNET_FRAME {
        (E_INVAL, false)
    } else {
        match send(driver, body) {
            Ok(()) => (0, false),
            Err(TxError::Full) => (E_AGAIN, false),
            Err(TxError::TooLong) => (E_INVAL, false),
            Err(TxError::Stalled) => (E_IO, true),
        }
    };
    let _ = reply_with_status(sender_pid, tx, req, status);
    stalled
}
