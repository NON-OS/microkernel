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
//! Which deauthentication or disassociation the link takes.

use super::super::super::rx_data::RxMpdu;
use nonos_wifi_core::dot11::auth::parse_leave;
use nonos_wifi_core::dot11::ccmp::CCMP_HDR_LEN;

/// The reason code if `rx` is the access point's deauthentication or
/// disassociation of this station that the link must take.
pub fn leave_reason(rx: &RxMpdu, us: &[u8; 6], bssid: &[u8; 6], pmf: bool) -> Option<u16> {
    let f = &rx.frame;
    let protected = f.get(1).is_some_and(|b| b & 0x40 != 0);
    match (pmf, protected) {
        (false, false) => parse_leave(f, us, bssid),
        (true, true) if rx.decrypted => {
            // The reason follows the CCMP header the firmware left in place.
            let mut clear = f.get(..24)?.to_vec();
            clear.extend_from_slice(f.get(24 + CCMP_HDR_LEN..)?);
            parse_leave(&clear, us, bssid)
        }
        _ => None,
    }
}
