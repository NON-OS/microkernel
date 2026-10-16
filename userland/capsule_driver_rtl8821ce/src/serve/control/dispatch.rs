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

//! One control request, checked and answered.

use nonos_wifi_core::wipe::wipe;

use super::super::connect::Session;
use super::super::radio::Radio;
use super::super::scanner::Scanner;
use super::super::Stage;
use super::join::connect_reply;
use super::leave::{code_reply, disconnect_reply};
use super::ops::{OP_CONNECT, OP_DISCONNECT, OP_LINK, OP_SCAN, OP_STATUS, WIFI_HDR, WIFI_MAGIC};
use super::scan::scan_reply;
use super::{link_op, status_op};

/// Handle one control request. Returns the reply length, or `None` if it is not a
/// well-formed control frame (so net_core traffic is never misread as control).
pub(in super::super) fn control(
    req: &mut [u8],
    radio: &mut Radio,
    session: &mut Option<Session>,
    scanner: &Scanner,
    stage: Stage,
    out: &mut [u8],
) -> Option<usize> {
    if req.len() < WIFI_HDR || u32::from_le_bytes([req[0], req[1], req[2], req[3]]) != WIFI_MAGIC {
        return None;
    }
    let op = u16::from_le_bytes([req[4], req[5]]);
    let rid = u32::from_le_bytes([req[6], req[7], req[8], req[9]]);
    out[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&op.to_le_bytes());
    out[6..10].copy_from_slice(&rid.to_le_bytes());

    match op {
        OP_STATUS => status_op::status_reply(stage, radio, out),
        OP_LINK => link_op::link_reply(radio, session.as_ref(), out),
        OP_SCAN => scan_reply(radio, scanner, out),
        OP_CONNECT => {
            let len = connect_reply(&req[WIFI_HDR..], radio, session, out);
            // The body carries the passphrase: gone once the join has run.
            wipe(&mut req[WIFI_HDR..]);
            len
        }
        OP_DISCONNECT => disconnect_reply(radio, session, out),
        _ => code_reply(-1, out),
    }
}
