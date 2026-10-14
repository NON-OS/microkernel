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

//! The connect body, read with every length checked.

use super::codes::{FLAG_HIDDEN, FLAG_REJOIN, FLAG_WPA3_ONLY, SSID_MAX};

/// A parsed connect request.
pub struct ConnectRequest<'a> {
    pub ssid: &'a [u8],
    pub pass: &'a [u8],
    pub wpa3_only: bool,
    pub hidden: bool,
    /// net.core's rejoin of a lost link (`LinkHistory::refuses_rejoin`).
    pub rejoin: bool,
}

/// Parse a connect body, or `None` if a length runs past it or the SSID is
/// empty or longer than 32 octets.
pub fn parse_connect(body: &[u8]) -> Option<ConnectRequest<'_>> {
    let ssid_len = *body.first()? as usize;
    if ssid_len == 0 || ssid_len > SSID_MAX {
        return None;
    }
    let ssid = body.get(1..1 + ssid_len)?;
    let pass_off = 1 + ssid_len;
    let pass_len = *body.get(pass_off)? as usize;
    let pass_end = pass_off + 1 + pass_len;
    let pass = body.get(pass_off + 1..pass_end)?;
    let flags = body.get(pass_end).copied().unwrap_or(0);
    let (wpa3_only, hidden, rejoin) = (flags & FLAG_WPA3_ONLY != 0, flags & FLAG_HIDDEN != 0, flags & FLAG_REJOIN != 0);
    Some(ConnectRequest { ssid, pass, wpa3_only, hidden, rejoin })
}
