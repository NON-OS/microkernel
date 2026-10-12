/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The connect reply: how a join went and how far its handshake got.

/// A join's result: the driver status code (0 joined, negative a named
/// failure, see `join_text`), how far the handshake got, the AKM that ran
/// and the access point's own status or reason code.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

/// Parse a connect reply body, or `None` when it does not reach the status.
pub fn parse_connect_reply(body: &[u8]) -> Option<ConnectResult> {
    let code = le32(body, 0)? as i32;
    if body.len() < 33 {
        return Some(ConnectResult { code, ..Default::default() });
    }
    Some(ConnectResult {
        code,
        sent: le32(body, 4)?,
        recv: le32(body, 8)?,
        data: le32(body, 12)?,
        eapol: le32(body, 16)?,
        probe: le32(body, 20)?,
        deauth: le32(body, 24)?,
        to_us: le32(body, 28)?,
        state: body[32],
        akm: body.get(33).copied().unwrap_or(0),
        ap_code: body.get(34..36).map_or(0, |b| u16::from_le_bytes([b[0], b[1]])),
    })
}

fn le32(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}
