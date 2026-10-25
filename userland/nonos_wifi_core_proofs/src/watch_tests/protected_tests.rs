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

//! Management frames protected under the pairwise key. The pinned frame is a
//! deauthentication (reason 3, PN 2) the Python `cryptography` AES-CCM made
//! with hostapd's ccmp_aad_nonce() rules for a management frame (its nonce's
//! Management flag, the masked frame control and sequence control), so the
//! station's own CCMP is not the only judge of its management path.

use super::frames::{mgmt, protected, sa_query_of, ACTION, DEAUTH};
use super::sim::{h, joined, STA};
use nonos_wifi_core::station::{Loss, Rx, RxDrop};

const PINNED_DEAUTH: &str = "c0400000 020000000002 020000000001 020000000001 3001
    0200002000000000 105e9f20a4f6063ee000";

#[test]
fn the_pinned_protected_deauthentication_ends_the_link_and_a_changed_one_does_not() {
    let pinned = h(PINNED_DEAUTH);
    let mut s = joined(true);
    let mut forged = pinned.clone();
    *forged.last_mut().unwrap() ^= 1;
    assert!(matches!(s.receive(&forged, false), Rx::Dropped(RxDrop::Undecryptable)));
    assert!(s.is_associated());
    assert!(matches!(s.receive(&pinned, false), Rx::Lost(Loss::Left(3))));
    assert!(!s.is_associated());
}

#[test]
fn a_replayed_protected_deauthentication_is_dropped() {
    let mut s = joined(true);
    let query = protected(&mgmt(ACTION, STA, &[8, 0, 0x34, 0x12]), 5);
    assert!(matches!(s.receive(&query, false), Rx::Send(_)));
    for pn in [5, 4, 1] {
        let replay = protected(&mgmt(DEAUTH, STA, &[3, 0]), pn);
        assert!(matches!(s.receive(&replay, false), Rx::Dropped(RxDrop::Replay)), "PN {pn}");
        assert!(s.is_associated());
    }
    let fresh = protected(&mgmt(DEAUTH, STA, &[3, 0]), 6);
    assert!(matches!(s.receive(&fresh, false), Rx::Lost(Loss::Left(3))));
}

#[test]
fn the_access_point_s_sa_query_is_answered_under_the_pairwise_key() {
    let mut s = joined(true);
    let ask = protected(&mgmt(ACTION, STA, &[8, 0, 0x34, 0x12]), 1);
    let Rx::Send(answer) = s.receive(&ask, false) else { panic!("unanswered") };
    let q = sa_query_of(&answer).unwrap();
    assert!(q.response && q.id == 0x1234);
    let in_clear = mgmt(ACTION, STA, &[8, 0, 0x35, 0x12]);
    assert!(matches!(s.receive(&in_clear, false), Rx::Dropped(RxDrop::Management)));
    let mut no_pmf = joined(false);
    let ask = protected(&mgmt(ACTION, STA, &[8, 0, 0x36, 0x12]), 2);
    assert!(matches!(no_pmf.receive(&ask, false), Rx::Dropped(RxDrop::Management)));
}
