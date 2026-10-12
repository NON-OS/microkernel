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

//! The SA Query action frames (IEEE Std 802.11-2020, 9.6.9 and 11.13). With
//! management frame protection on, a station told in the clear that it was
//! deauthenticated asks the access point, under the pairwise key, whether the
//! association still stands, and the access point may ask the same of the
//! station. A request and its response share a transaction identifier.

use alloc::vec::Vec;

use super::header::{frame_control, write_header, MacAddr, MAC_HEADER_LEN, TYPE_MGMT};

/// The Action management frame subtype.
pub const SUBTYPE_ACTION: u8 = 13;
/// The SA Query category. It is a robust one, so it is only sent protected.
pub const CATEGORY_SA_QUERY: u8 = 8;
const ACTION_REQUEST: u8 = 0;
const ACTION_RESPONSE: u8 = 1;
/// Category, action and the transaction identifier.
const BODY_LEN: usize = 4;

/// An SA Query request or response and its transaction identifier.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SaQuery {
    pub response: bool,
    pub id: u16,
}

/// The plaintext SA Query frame `q` from `src` to the access point `bssid`,
/// for the caller to protect under the pairwise key.
pub fn sa_query_frame(src: MacAddr, bssid: MacAddr, seq: u16, q: SaQuery) -> Vec<u8> {
    let mut out = alloc::vec![0u8; MAC_HEADER_LEN + BODY_LEN];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_ACTION);
    // The buffer is sized for the header, so this cannot refuse.
    let _ = write_header(&mut out, fc, bssid, src, bssid, seq);
    let action = if q.response { ACTION_RESPONSE } else { ACTION_REQUEST };
    out[MAC_HEADER_LEN] = CATEGORY_SA_QUERY;
    out[MAC_HEADER_LEN + 1] = action;
    out[MAC_HEADER_LEN + 2..].copy_from_slice(&q.id.to_le_bytes());
    out
}

/// The SA Query in an Action frame's plaintext body, or `None` for any other
/// action or a body too short to hold one.
pub fn parse_sa_query(body: &[u8]) -> Option<SaQuery> {
    let b = body.get(..BODY_LEN)?;
    if b[0] != CATEGORY_SA_QUERY {
        return None;
    }
    let response = match b[1] {
        ACTION_REQUEST => false,
        ACTION_RESPONSE => true,
        _ => return None,
    };
    Some(SaQuery { response, id: u16::from_le_bytes([b[2], b[3]]) })
}
