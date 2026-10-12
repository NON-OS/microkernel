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

//! Parse an anti-clogging token request body: group 19, then the token (in
//! its container element for H2E).

use super::*;

/// Parse an anti-clogging token request body: group 19, then the token (in
/// its container element for H2E). Returns the token.
pub fn parse_token_request(body: &[u8], h2e: bool) -> Option<&[u8]> {
    if body.len() < 3 || u16::from_le_bytes([body[0], body[1]]) != GROUP_19 {
        return None;
    }
    let rest = &body[2..];
    let token = if h2e {
        if rest.len() < 3 || rest[0] != EID_EXTENSION || rest[2] != EID_EXT_ANTI_CLOGGING_TOKEN {
            return None;
        }
        let len = rest[1] as usize;
        if len < 2 || 2 + len > rest.len() {
            return None;
        }
        &rest[3..2 + len]
    } else {
        rest
    };
    (!token.is_empty() && token.len() <= TOKEN_MAX).then_some(token)
}

/// Parse a confirm body: the peer's send-confirm and confirm value.
pub fn parse_confirm(body: &[u8]) -> Option<(u16, [u8; 32])> {
    if body.len() < 2 + 32 {
        return None;
    }
    let mut c = [0u8; 32];
    c.copy_from_slice(&body[2..34]);
    Some((u16::from_le_bytes([body[0], body[1]]), c))
}
