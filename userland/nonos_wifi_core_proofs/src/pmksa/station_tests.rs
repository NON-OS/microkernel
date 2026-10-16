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

//! The soft-MAC station's cache over its life: a WPA2-PSK join keeps no
//! PMKSA, an SAE PMKSA it holds outlives the session, and the person's
//! disconnect wipes every one, joined or not.

use crate::soft::join::joined;
use crate::soft::wire::{code, control};
use nonos_wifi_core::pmksa::Pmksa;
use nonos_wifi_core::softmac::engine::OP_DISCONNECT;

const HELD: Pmksa = Pmksa { bssid: [2, 0, 0, 0, 0, 7], pmk: [7; 32], pmkid: [8; 16] };

#[test]
fn a_psk_join_keeps_no_pmksa() {
    let mut st = joined();
    assert!(st.pmksa().is_empty());
}

#[test]
fn the_persons_disconnect_wipes_every_pmksa() {
    let mut st = joined();
    st.pmksa().keep(b"home", b"pass", HELD, 0);
    st.pmksa().keep(b"work", b"pass", Pmksa { bssid: [2, 0, 0, 0, 0, 9], ..HELD }, 0);
    assert_eq!(code(&control(&mut st, OP_DISCONNECT, &[])), 0);
    assert!(st.pmksa().is_empty(), "joined");
    st.pmksa().keep(b"home", b"pass", HELD, 0);
    assert_eq!(code(&control(&mut st, OP_DISCONNECT, &[])), 0);
    assert!(st.pmksa().is_empty(), "and with no session to leave");
}
