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

//! Build a commit body: group 19, the token (raw for hunting and pecking),
//! the scalar and element, then the token in its container for H2E.

use super::*;

/// Build a commit body: group 19, the token (raw for hunting and pecking),
/// the scalar and element, then the token in its container for H2E.
pub fn commit_body(
    scalar: &[u8; LEN],
    element: &[u8; 2 * LEN],
    token: &[u8],
    h2e: bool,
) -> Vec<u8> {
    let mut b = Vec::with_capacity(2 + 3 * LEN + 3 + token.len());
    b.extend_from_slice(&GROUP_19.to_le_bytes());
    if !h2e {
        b.extend_from_slice(token);
    }
    b.extend_from_slice(scalar);
    b.extend_from_slice(element);
    if h2e && !token.is_empty() {
        b.push(EID_EXTENSION);
        b.push((token.len() + 1) as u8);
        b.push(EID_EXT_ANTI_CLOGGING_TOKEN);
        b.extend_from_slice(token);
    }
    b
}

/// Build a confirm body: the send-confirm counter and the confirm value.
pub fn confirm_body(send_confirm: u16, confirm: &[u8; 32]) -> Vec<u8> {
    let mut b = Vec::with_capacity(2 + 32);
    b.extend_from_slice(&send_confirm.to_le_bytes());
    b.extend_from_slice(confirm);
    b
}

/// Parse the AP's commit body. Elements after the element (a password
/// identifier, rejected groups) are not used by a group-19-only station.
pub fn parse_commit(body: &[u8]) -> Option<PeerCommit> {
    if body.len() < 2 + 3 * LEN || u16::from_le_bytes([body[0], body[1]]) != GROUP_19 {
        return None;
    }
    let mut scalar_bytes = [0u8; LEN];
    scalar_bytes.copy_from_slice(&body[2..2 + LEN]);
    let mut element_bytes = [0u8; 2 * LEN];
    element_bytes.copy_from_slice(&body[2 + LEN..2 + 3 * LEN]);
    let scalar = scalar_from_bytes(&scalar_bytes)?;
    if scalar_is_trivial(&scalar) {
        return None;
    }
    let mut x = [0u8; LEN];
    let mut y = [0u8; LEN];
    x.copy_from_slice(&element_bytes[..LEN]);
    y.copy_from_slice(&element_bytes[LEN..]);
    let element = point_from_xy(&x, &y)?;
    Some(PeerCommit { scalar, scalar_bytes, element, element_bytes })
}
