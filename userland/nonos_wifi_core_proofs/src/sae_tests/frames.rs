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

//! A reflected commit, the other PWE method, and an anti-clogging request.

use nonos_wifi_core::sae::frame::{commit_body, STATUS_SAE_HASH_TO_ELEMENT};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep};

use super::known::{hnp_commit, ADDR1, ADDR2, PW};

#[test]
fn a_reflected_commit_and_a_mismatched_method_are_dropped() {
    let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
    let mut sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    let (_, status, own) = sta.commit_frame();
    assert!(matches!(sta.on_frame(1, status, &own), SaeStep::Ignore), "reflection dropped");
    assert_eq!(sta.state(), SaeState::Committed);
    // The AP answering a hunting-and-pecking commit with the H2E status code.
    let c = hnp_commit();
    let other = commit_body(&c.scalar(), &c.element().unwrap(), &[], false);
    assert!(matches!(sta.on_frame(1, STATUS_SAE_HASH_TO_ELEMENT, &other), SaeStep::Ignore));
    assert_eq!((sta.state(), sta.failure()), (SaeState::Committed, Some(SaeFailure::BadCommit)));
}

#[test]
fn an_anti_clogging_request_is_answered_with_the_token() {
    for h2e in [false, true] {
        let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
        let mut sta = SaeStation::start(pwe, h2e, &[0x11; 32], &[0x22; 32]).unwrap();
        let token = [0xABu8; 34];
        let mut req = vec![0x13, 0x00];
        if h2e {
            req.extend_from_slice(&[255, 35, 93]);
        }
        req.extend_from_slice(&token);
        let SaeStep::Send { seq, body, .. } = sta.on_frame(1, 76, &req) else {
            panic!("the commit is sent again");
        };
        assert_eq!(seq, 1);
        assert!(body.windows(token.len()).any(|w| w == token), "the token rides along, h2e={h2e}");
        assert_eq!(body.len(), 2 + 96 + 34 + if h2e { 3 } else { 0 });
    }
}
