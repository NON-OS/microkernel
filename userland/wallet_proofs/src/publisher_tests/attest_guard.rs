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

//! The attest's presses: a row only opens the review, the review's Sign alone
//! signs, under the focus guard, and only the digest the review showed.

use crate::publisher::{attest_act, may_sign, AttestAct};
use crate::wallet::arm::moves::{moves, Shown};
use crate::wallet::arm::rows::{ROW_ATTEST, ROW_BINDING, ROW_INTENT};
use crate::wallet::screen::hits::Press::{Back, Field, Footer, Row};

#[test]
fn a_row_only_opens_the_review_and_only_its_sign_signs() {
    for i in 0..16u8 {
        assert_eq!(attest_act(None, Row(ROW_ATTEST + i)), AttestAct::Open(i as usize));
    }
    for p in [Footer(0), Footer(1), Back, Field(0), Row(ROW_BINDING), Row(ROW_INTENT)] {
        assert_eq!(attest_act(None, p), AttestAct::Other, "no review up: not an attest press");
    }
    assert_eq!(attest_act(Some(3), Footer(0)), AttestAct::Sign(3));
    assert_eq!(attest_act(Some(3), Footer(1)), AttestAct::Cancel);
    assert_eq!(attest_act(Some(3), Back), AttestAct::Cancel);
    assert_eq!(attest_act(Some(3), Row(ROW_ATTEST)), AttestAct::Other, "rows under it sleep");
}

#[test]
fn the_reviews_sign_is_guarded_and_cancel_is_not() {
    assert!(moves(Shown::AttestReview, Some(Footer(0))), "Sign the attest");
    for p in [Some(Footer(1)), Some(Back), Some(Row(ROW_ATTEST)), None] {
        assert!(!moves(Shown::AttestReview, p), "{:?} acts at once", p.is_some());
    }
}

#[test]
fn sign_signs_only_the_digest_its_review_showed() {
    let (a, b) = (Some([1u8; 32]), Some([2u8; 32]));
    assert!(may_sign(a, a));
    assert!(!may_sign(a, b), "the attest changed after it was shown");
    assert!(!may_sign(None, a), "nothing was shown");
    assert!(!may_sign(a, None), "it can no longer be made");
    assert!(!may_sign(None, None));
}
