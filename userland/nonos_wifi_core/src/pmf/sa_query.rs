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

//! SA Query action frames (IEEE Std 802.11-2020, 9.6.9): a request and a
//! response that echo a two-octet transaction identifier. With management
//! frame protection on they are robust, so they travel protected under the
//! pairwise key, and an answer proves the peer still holds the association.

use alloc::vec::Vec;

use crate::dot11::header::{frame_control, write_header, MacAddr, MAC_HEADER_LEN, TYPE_MGMT};

pub const SUBTYPE_ACTION: u8 = 13;
pub const CATEGORY_SA_QUERY: u8 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaQueryAction {
    Request = 0,
    Response = 1,
}

/// An SA Query frame from `src` to the access point `bssid`, unprotected:
/// the link protects it on the way out.
pub fn sa_query_frame(
    src: MacAddr,
    bssid: MacAddr,
    seq_ctrl: u16,
    action: SaQueryAction,
    txid: [u8; 2],
) -> Vec<u8> {
    let mut out = alloc::vec![0u8; MAC_HEADER_LEN + 4];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_ACTION);
    // The buffer is sized for the header, so this cannot refuse.
    let _ = write_header(&mut out, fc, bssid, src, bssid, seq_ctrl);
    out[MAC_HEADER_LEN..].copy_from_slice(&[CATEGORY_SA_QUERY, action as u8, txid[0], txid[1]]);
    out
}

/// The action and transaction identifier of an SA Query action body (the
/// plaintext after the MAC header), or `None` for any other action.
pub fn parse_sa_query(body: &[u8]) -> Option<(SaQueryAction, [u8; 2])> {
    if body.len() < 4 || body[0] != CATEGORY_SA_QUERY {
        return None;
    }
    let action = match body[1] {
        0 => SaQueryAction::Request,
        1 => SaQueryAction::Response,
        _ => return None,
    };
    Some((action, [body[2], body[3]]))
}
