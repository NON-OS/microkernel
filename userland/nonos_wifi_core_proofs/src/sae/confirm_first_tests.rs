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

//! No key is out before the AP's confirm verifies: not after the AP's commit,
//! not from a confirm that comes before any commit, not from one that fails.

use super::bound_tests::pair;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStep};

#[test]
fn no_key_is_out_before_the_aps_confirm_verifies() {
    let (mut sta, mut ap) = pair(0x44);
    let (_, st, sta_commit) = sta.commit_frame();
    let (_, ap_st, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: mut c, .. } = ap.on_frame(1, st, &sta_commit) else { panic!() };
    assert!(matches!(sta.on_frame(2, 0, &c), SaeStep::Ignore), "a confirm before any commit");
    sta.on_frame(1, ap_st, &ap_commit);
    assert_eq!((sta.state(), sta.pmk()), (SaeState::Confirmed, None));
    c[10] ^= 1;
    assert!(matches!(sta.on_frame(2, 0, &c), SaeStep::Ignore));
    assert_eq!((sta.failure(), sta.pmk()), (Some(SaeFailure::BadConfirm), None));
}
