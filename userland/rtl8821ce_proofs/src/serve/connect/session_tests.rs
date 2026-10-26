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

//! A session keeps what its association installed: the pairwise key's slot,
//! the network's name cut to 32 bytes, the AKM that ran, and the group key's
//! slot as the AP's rekeys move it.

use super::consts::{GROUP_KEY_SLOTS, JOIN_BUDGET_MS, PAIRWISE_KEY_ID};
use super::session::Session;

#[test]
fn a_session_keeps_what_its_association_installed() {
    let mut s = Session::new([7; 6], 1, &[b'n'; 40], 8);
    assert_eq!((s.key_id, s.ssid_len, s.akm), (PAIRWISE_KEY_ID, 32, 8), "a name cut at 32 bytes");
    assert_eq!(s.ssid, [b'n'; 32]);
    s.set_gtk_id(2);
    assert_eq!(s.gtk_id, 2, "the group key moved with the AP's rekey");
}

// The join and its hunt return inside the panel's 30 s connect call, and a
// CCMP KeyID, two bits, names one of the four group key slots.
const _: () = assert!(JOIN_BUDGET_MS < 30_000 && GROUP_KEY_SLOTS == 4);
