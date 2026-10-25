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

//! The connect reply: the status code, the exchange's counts and the codes.

use super::codes::{CONNECT_REPLY, WIFI_HDR};
use crate::firmware::gen3::join::exchange::Progress;

/// A join's result for the reply.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ConnectResult {
    pub code: i32,
    pub progress: Progress,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

impl ConnectResult {
    pub fn code(code: i32) -> Self {
        Self { code, ..Default::default() }
    }
}

/// Write the connect reply after the header in `out`; its length with the
/// header, or `None` when `out` is too small.
pub fn encode_connect(out: &mut [u8], r: &ConnectResult) -> Option<usize> {
    let b = out.get_mut(WIFI_HDR..WIFI_HDR + CONNECT_REPLY)?;
    let p = &r.progress;
    let words = [r.code as u32, p.sent, p.recv, p.data, p.eapol, p.probe, p.deauth, p.to_us];
    for (i, w) in words.iter().enumerate() {
        b[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    b[32] = r.state;
    b[33] = r.akm;
    b[34..36].copy_from_slice(&r.ap_code.to_le_bytes());
    Some(WIFI_HDR + CONNECT_REPLY)
}
