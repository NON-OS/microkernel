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

//! The Wi-Fi control family setup, Settings and net_core speak through
//! `nonos_wifi_client`, tagged with its own magic so it never collides with
//! the link protocol: `[magic u32][op u16][request id u32]` then the body, and
//! the reply repeats the header. Status, link and scan answer at once;
//! connect runs the whole join.

use super::station::SoftStation;
use crate::softmac::control::result::CODE_REFUSED;
use crate::softmac::control::ConnectResult;
use crate::softmac::radio::{Env, Radio};

pub const WIFI_MAGIC: u32 = 0x5749_4649;
pub const WIFI_HDR: usize = 10;
pub const OP_CONNECT: u16 = 1;
pub const OP_DISCONNECT: u16 = 2;
pub const OP_SCAN: u16 = 3;
pub const OP_STATUS: u16 = 4;
pub const OP_LINK: u16 = 5;
/// Turn the radio on or off (E-72): body octet 0 off, 1 on.
pub const OP_RADIO: u16 = 6;

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn control(&mut self, req: &[u8], out: &mut [u8]) -> Option<usize> {
        if req.len() < WIFI_HDR || req[0..4] != WIFI_MAGIC.to_le_bytes() || out.len() < 512 {
            return None;
        }
        let op = u16::from_le_bytes([req[4], req[5]]);
        out[..WIFI_HDR].copy_from_slice(&req[..WIFI_HDR]);
        let n = match op {
            OP_STATUS => self.status_reply(&mut out[WIFI_HDR..]),
            OP_LINK => self.link_reply(&mut out[WIFI_HDR..]),
            OP_SCAN => self.scan_reply(&mut out[WIFI_HDR..]),
            // A radio turned off runs no join and sends no probe (E-72).
            OP_CONNECT if self.off => {
                ConnectResult::code(CODE_REFUSED).encode(&mut out[WIFI_HDR..])
            }
            OP_CONNECT => self.connect(&req[WIFI_HDR..]).encode(&mut out[WIFI_HDR..]),
            OP_DISCONNECT => self.disconnect(&mut out[WIFI_HDR..]),
            OP_RADIO => self.set_radio(&req[WIFI_HDR..], &mut out[WIFI_HDR..]),
            _ => code(-1, &mut out[WIFI_HDR..]),
        };
        Some(WIFI_HDR + n)
    }

    fn disconnect(&mut self, out: &mut [u8]) -> usize {
        // The person's disconnect forgets every PMKSA, as it should.
        self.pmksa.wipe_all();
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) else {
            return code(if self.radio.is_some() { 0 } else { -1 }, out);
        };
        s.leave(r);
        self.end_session();
        code(0, out)
    }
}

pub(super) fn code(status: i32, out: &mut [u8]) -> usize {
    out[..4].copy_from_slice(&status.to_le_bytes());
    4
}
