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

//! SAE against a hostile AP: what it can make the station do again is
//! bounded. Anti-clogging requests are answered at most three times; the
//! AP's commit sent again draws this station's confirm again, its counter
//! one higher, at most three confirms; and a flood of other commits once
//! this station has confirmed is ignored at no cost. Past a bound the frame
//! is discarded, and the exchange runs on to the AP's real confirm.

use crate::sae::vectors::{ADDR1, ADDR2, PW};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::station::{CONFIRMS_MAX, TOKENS_MAX};
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep};

pub fn pair(ap_mask: u8) -> (SaeStation, SaeStation) {
    let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
    let sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    (sta, SaeStation::start(pwe, false, &[0x33; 32], &[ap_mask; 32]).unwrap())
}

fn counter(step: SaeStep) -> Option<u16> {
    match step {
        SaeStep::Send { seq: 2, body, .. } => Some(u16::from_le_bytes([body[0], body[1]])),
        _ => None,
    }
}

#[test]
fn anti_clogging_requests_are_answered_three_times_and_no_more() {
    let (mut sta, _) = pair(0x44);
    let req = [&[0x13, 0x00][..], &[0xAB; 32]].concat();
    for _ in 0..TOKENS_MAX {
        assert!(matches!(sta.on_frame(1, 76, &req), SaeStep::Send { seq: 1, .. }));
    }
    assert!(matches!(sta.on_frame(1, 76, &req), SaeStep::Ignore));
    assert_eq!(sta.failure(), Some(SaeFailure::Looping));
    assert_eq!(sta.state(), SaeState::Committed, "the exchange runs on");
}

#[test]
fn the_aps_commit_again_draws_a_counted_confirm_three_times_and_no_more() {
    let (mut sta, mut ap) = pair(0x44);
    let (_, st, sta_commit) = sta.commit_frame();
    let (_, ap_st, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, st, &sta_commit) else { panic!() };
    assert_eq!(counter(sta.on_frame(1, ap_st, &ap_commit)), Some(1));
    // Other commits now, a flood of them, are no one's this exchange knows.
    for mask in 0x50..0x90u8 {
        let (_, _, other) = pair(mask).1.commit_frame();
        assert!(matches!(sta.on_frame(1, ap_st, &other), SaeStep::Ignore));
    }
    for n in 2..=CONFIRMS_MAX {
        assert_eq!(counter(sta.on_frame(1, ap_st, &ap_commit)), Some(n), "the AP lost ours");
    }
    assert!(matches!(sta.on_frame(1, ap_st, &ap_commit), SaeStep::Ignore), "one too many");
    assert_eq!((sta.failure(), sta.pmk()), (Some(SaeFailure::Looping), None));
    let real = sta.on_frame(2, 0, &ap_confirm);
    assert!(matches!(real, SaeStep::Accepted { .. }), "the AP's own confirm still ends it");
}
