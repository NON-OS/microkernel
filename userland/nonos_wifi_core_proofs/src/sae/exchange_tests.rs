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

//! A whole SAE exchange, by hunting and pecking and by hash to element: the
//! station against an AP built from the same code agree on the PMK with the
//! same password and fail at the confirm with another.

use nonos_wifi_core::sae::frame::{STATUS_SAE_HASH_TO_ELEMENT, STATUS_SUCCESS};
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep};

// Run a whole exchange: the station against an AP built from the same code
// with its own rand and mask. Both must end Accepted with the same PMK.
fn exchange(h2e: bool, sta_pw: &[u8], ap_pw: &[u8]) -> (SaeStation, SaeStation) {
    let (sta_mac, ap_mac) = ([0x02, 0, 0, 0, 0, 1], [0x02, 0, 0, 0, 0, 2]);
    let pwe = |pw: &[u8]| {
        if h2e {
            pwe_from_pt(&derive_pt(b"home", pw, None).unwrap(), &sta_mac, &ap_mac).unwrap()
        } else {
            derive_pwe(pw, &vec![7u8; pw.len()], &sta_mac, &ap_mac).unwrap()
        }
    };
    let mut sta = SaeStation::start(pwe(sta_pw), h2e, &[0x11; 32], &[0x22; 32]).unwrap();
    let mut ap = SaeStation::start(pwe(ap_pw), h2e, &[0x33; 32], &[0x44; 32]).unwrap();
    let (_, status, sta_commit) = sta.commit_frame();
    let want = if h2e { STATUS_SAE_HASH_TO_ELEMENT } else { STATUS_SUCCESS };
    assert_eq!(status, want, "the commit's status code names the PWE method");
    let (_, ap_status, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, status, &sta_commit) else {
        panic!("the AP answers a commit with a confirm");
    };
    let SaeStep::Send { seq, body: sta_confirm, .. } = sta.on_frame(1, ap_status, &ap_commit)
    else {
        panic!("the station answers the AP's commit with a confirm");
    };
    assert_eq!(seq, 2);
    ap.on_frame(2, 0, &sta_confirm);
    sta.on_frame(2, 0, &ap_confirm);
    (sta, ap)
}

#[test]
fn an_exchange_with_the_same_password_agrees_on_the_pmk() {
    for h2e in [false, true] {
        let (sta, ap) = exchange(h2e, b"correct horse", b"correct horse");
        assert_eq!(sta.state(), SaeState::Accepted, "h2e={h2e}");
        assert_eq!(ap.state(), SaeState::Accepted, "h2e={h2e}");
        assert_eq!(sta.pmk().unwrap(), ap.pmk().unwrap(), "same PMK and PMKID, h2e={h2e}");
    }
}

#[test]
fn a_different_password_fails_at_the_confirm() {
    for h2e in [false, true] {
        let (sta, _) = exchange(h2e, b"correct horse", b"wrong horse!!");
        assert_eq!(sta.state(), SaeState::Failed, "h2e={h2e}");
        assert_eq!(sta.failure(), Some(SaeFailure::BadConfirm));
        assert!(sta.pmk().is_none(), "no PMK from a failed exchange");
    }
}
