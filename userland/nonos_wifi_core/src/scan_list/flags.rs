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

//! The secured, WPA2 and WPA3 bits the panel shows, read from a beacon's RSNE.

use super::*;

pub fn beacon_flags(frame: &[u8], capability: u16) -> u8 {
    use crate::dot11::ies::{beacon_tags, find_element, EID_RSN};
    use crate::rsn::parse_rsne;
    let rsne =
        beacon_tags(frame).and_then(|t| find_element(t, EID_RSN)).and_then(|e| parse_rsne(&e[2..]));
    let mut flags = if rsne.is_some() || capability & 0x0010 != 0 { FLAG_SECURED } else { 0 };
    if let Some(r) = rsne {
        if r.akm_psk || r.akm_psk_sha256 {
            flags |= FLAG_WPA2;
        }
        if r.akm_sae {
            flags |= FLAG_WPA3;
        }
    }
    flags
}
