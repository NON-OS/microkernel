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

//! The SA Query a frame the station sent carries, read under `TK`: whether
//! it is a response, and its transaction identifier as a number.

use super::sim::{AP, TK};
use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::pmf::{parse_sa_query, SaQueryAction};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SaQuery {
    pub response: bool,
    pub id: u16,
}

/// The SA Query in a frame the station sent, read under `TK`.
pub fn sa_query_of(frame: &[u8]) -> Option<SaQuery> {
    assert_eq!(frame[4..10], AP, "sent to the access point");
    let (action, txid) = parse_sa_query(&decrypt(frame, &TK)?)?;
    Some(SaQuery { response: action == SaQueryAction::Response, id: u16::from_le_bytes(txid) })
}
