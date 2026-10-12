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

//! What the device sends unasked on the control channel, once its
//! notification endpoint says a response is waiting (Remote NDIS 1.0,
//! 2.2.10 and 2.2.11, as recalled, unverified): INDICATE_STATUS with a
//! media state, and KEEPALIVE, which the host answers.

use super::message::{le32, put32, COMPLETION, MSG_INDICATE, MSG_KEEPALIVE, STATUS_SUCCESS};
use super::message::{STATUS_MEDIA_CONNECT, STATUS_MEDIA_DISCONNECT};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FromDevice {
    /// The medium connected (true) or disconnected.
    Media(bool),
    /// A KEEPALIVE under this RequestID, to be answered.
    KeepAlive(u32),
    Other,
}

/// The message in `r`, its MessageLength checked against what came.
pub fn from_device(r: &[u8]) -> FromDevice {
    let (Some(kind), Some(len)) = (le32(r, 0), le32(r, 4)) else { return FromDevice::Other };
    if (len as usize) > r.len() || len < 12 {
        return FromDevice::Other;
    }
    match (kind, le32(r, 8)) {
        (MSG_INDICATE, Some(STATUS_MEDIA_CONNECT)) => FromDevice::Media(true),
        (MSG_INDICATE, Some(STATUS_MEDIA_DISCONNECT)) => FromDevice::Media(false),
        (MSG_KEEPALIVE, Some(id)) => FromDevice::KeepAlive(id),
        _ => FromDevice::Other,
    }
}

/// KEEPALIVE_CMPLT for RequestID `id`: success, 16 bytes.
pub fn keepalive_done(id: u32) -> [u8; 16] {
    let mut m = [0u8; 16];
    for (at, v) in [(0, MSG_KEEPALIVE | COMPLETION), (4, 16), (8, id), (12, STATUS_SUCCESS)] {
        put32(&mut m, at, v);
    }
    m
}
