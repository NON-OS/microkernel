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

//! The SAE fields of an Authentication frame body (IEEE Std 802.11-2020,
//! 9.3.3.12, Table 9-41): after the algorithm (3), transaction sequence and
//! status code, a commit carries the finite cyclic group, an anti-clogging
//! token when the AP asked for one, the scalar and the element; a confirm
//! carries the send-confirm counter and the confirm value. With
//! hash-to-element the token rides in an Anti-Clogging Token Container
//! element after the element instead. Parsing is bounded and refuses a group
//! other than 19, a scalar outside (1, r) and an element that is not a point.

use alloc::vec::Vec;

use p256::{ProjectivePoint, Scalar};

use super::group::{point_from_xy, scalar_from_bytes, scalar_is_trivial, GROUP_19, LEN};

/// The SAE authentication algorithm number.
pub const AUTH_ALG_SAE: u16 = 3;
/// Transaction sequence numbers.
pub const SEQ_COMMIT: u16 = 1;
pub const SEQ_CONFIRM: u16 = 2;
/// Status codes a commit carries or answers with (Table 9-50).
pub const STATUS_SUCCESS: u16 = 0;
pub const STATUS_ANTI_CLOGGING_TOKEN_REQ: u16 = 76;
pub const STATUS_GROUP_NOT_SUPPORTED: u16 = 77;
pub const STATUS_SAE_HASH_TO_ELEMENT: u16 = 126;
/// The longest anti-clogging token kept (hostapd sends 34 octets).
pub const TOKEN_MAX: usize = 64;

/// Element ids used inside SAE frames.
const EID_EXTENSION: u8 = 255;
const EID_EXT_ANTI_CLOGGING_TOKEN: u8 = 93;

/// A peer's commit, validated.
#[derive(Clone, Copy)]
pub struct PeerCommit {
    pub scalar: Scalar,
    pub scalar_bytes: [u8; LEN],
    pub element: ProjectivePoint,
    pub element_bytes: [u8; 2 * LEN],
}

/// Build a commit body: group 19, the token (raw for hunting and pecking),
/// the scalar and element, then the token in its container for H2E.
pub fn commit_body(scalar: &[u8; LEN], element: &[u8; 2 * LEN], token: &[u8], h2e: bool) -> Vec<u8> {
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
