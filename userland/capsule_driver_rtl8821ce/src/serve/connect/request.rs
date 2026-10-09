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

//! The connect request body: `[ssid_len][ssid][pass_len][pass]`, then an
//! optional flags octet. A client that predates the flags sends none, which
//! reads as "no constraints". Bounded: every length is checked against the
//! body before it is used, an SSID longer than 32 octets is refused, and
//! bytes past the flags are ignored.

/// The network was saved as WPA3: join it with SAE or not at all.
pub const FLAG_WPA3_ONLY: u8 = 1 << 0;
/// The network does not broadcast its name: a directed probe may carry it.
pub const FLAG_HIDDEN: u8 = 1 << 1;

/// A parsed connect request.
pub struct ConnectRequest<'a> {
    pub ssid: &'a [u8],
    pub pass: &'a [u8],
    pub wpa3_only: bool,
    pub hidden: bool,
}

/// Parse a connect request body, or `None` if a length runs past it or the
/// SSID is empty or longer than 32 octets.
pub fn parse_connect(body: &[u8]) -> Option<ConnectRequest<'_>> {
    let ssid_len = *body.first()? as usize;
    if ssid_len == 0 || ssid_len > 32 {
        return None;
    }
    let ssid = body.get(1..1 + ssid_len)?;
    let pass_off = 1 + ssid_len;
    let pass_len = *body.get(pass_off)? as usize;
    let pass_end = pass_off + 1 + pass_len;
    let pass = body.get(pass_off + 1..pass_end)?;
    let flags = body.get(pass_end).copied().unwrap_or(0);
    Some(ConnectRequest {
        ssid,
        pass,
        wpa3_only: flags & FLAG_WPA3_ONLY != 0,
        hidden: flags & FLAG_HIDDEN != 0,
    })
}
