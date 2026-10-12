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
//! Nothing before the access point's confirm verifies is authenticated, so a
//! forged frame is discarded (802.11-2020 12.4.8.6) and the real exchange
//! still completes after it.

use nonos_wifi_core::sae::frame::STATUS_SUCCESS;
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep};

const STA: [u8; 6] = [0x02, 0, 0, 0, 0, 1];
const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 2];

/// A station and an access point that know the same password, each with
/// its commit sent.
pub(super) fn pair() -> (SaeStation, SaeStation) {
    let pwe = derive_pwe(b"correct horse", &[7u8; 13], &STA, &AP).unwrap();
    let sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    let ap = SaeStation::start(pwe, false, &[0x33; 32], &[0x44; 32]).unwrap();
    (sta, ap)
}

/// Run the real exchange from where the station stands; true when accepted.
pub(super) fn finish(sta: &mut SaeStation, ap: &mut SaeStation) -> bool {
    let (_, _, sta_commit) = sta.commit_frame();
    let (_, _, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, STATUS_SUCCESS, &sta_commit) else {
        return false;
    };
    let _ = sta.on_frame(1, STATUS_SUCCESS, &ap_commit);
    matches!(sta.on_frame(2, STATUS_SUCCESS, &ap_confirm), SaeStep::Accepted)
}

#[test]
fn a_forged_refusal_or_group_rejection_is_discarded_and_the_exchange_completes() {
    for (status, why) in [(1, SaeFailure::Rejected(1)), (77, SaeFailure::GroupNotSupported)] {
        let (mut sta, mut ap) = pair();
        assert!(matches!(sta.on_frame(2, status, &[]), SaeStep::Ignore));
        assert!(matches!(sta.on_frame(1, status, &[]), SaeStep::Ignore));
        assert_eq!((sta.state(), sta.failure()), (SaeState::Committed, Some(why)));
        assert!(finish(&mut sta, &mut ap), "status {status}");
        assert_eq!(sta.failure(), None, "the hint goes once accepted");
    }
}

#[test]
fn a_forged_confirm_is_discarded_and_the_real_one_still_verifies() {
    let (mut sta, mut ap) = pair();
    let (_, _, sta_commit) = sta.commit_frame();
    let (_, _, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: real, .. } = ap.on_frame(1, STATUS_SUCCESS, &sta_commit) else {
        panic!("the AP confirms");
    };
    let _ = sta.on_frame(1, STATUS_SUCCESS, &ap_commit);
    let mut forged = real.clone();
    forged[5] ^= 1;
    assert!(matches!(sta.on_frame(2, STATUS_SUCCESS, &forged), SaeStep::Ignore));
    assert_eq!((sta.state(), sta.failure()), (SaeState::Confirmed, Some(SaeFailure::BadConfirm)));
    assert!(matches!(sta.on_frame(2, STATUS_SUCCESS, &real), SaeStep::Accepted));
    assert!(sta.pmk().is_some());
}
