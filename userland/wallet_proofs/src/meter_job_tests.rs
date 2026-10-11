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

//! The meter's jobs as the wallet reads them (v1.4.14), and a top-up typed.

use nonos_payment_proto::{TopUp, EAGAIN, EBUSY, EGAS, ENOENT, ETIME};

use crate::meter_job::{outcome, JobKind, Outcome};
use crate::meter_topup::{topup_request, NO_AMOUNT};

#[test]
fn a_job_runs_then_says_what_went_or_why_not() {
    assert_eq!(outcome(JobKind::TopUp, Err(EAGAIN)), Outcome::Running);
    let two = [[0xab; 32], [0xcd; 32]].concat();
    match outcome(JobKind::TopUp, Ok(&two)) {
        Outcome::Sent(line) => assert!(line.contains("0xabababab") && line.contains("0xcdcdcdcd")),
        other => panic!("{other:?}"),
    }
    assert!(matches!(outcome(JobKind::Close, Ok(&[1; 32])), Outcome::Sent(_)));
    assert!(matches!(outcome(JobKind::Close, Ok(&two)), Outcome::Failed(_)), "a close is one hash");
    for (kind, status, says) in [
        (JobKind::Close, ENOENT, "restarted"),
        (JobKind::TopUp, EBUSY, "busy"),
        (JobKind::TopUp, EGAS, "gas"),
        (JobKind::Close, ETIME, "grace"),
    ] {
        match outcome(kind, Err(status)) {
            Outcome::Failed(why) => assert!(why.contains(says), "{why}"),
            other => panic!("{other:?}"),
        }
    }
}

#[test]
fn a_top_up_is_whole_nox_within_bounds() {
    let e18 = 1_000_000_000_000_000_000u128;
    let t = topup_request(11_155_111, [4; 32], "50").unwrap();
    assert_eq!(t, TopUp { chain_id: 11_155_111, channel_id: [4; 32], amount: 50 * e18 });
    for bad in ["", "0", "1.5", "-1", "10000000000000", "1000000000001"] {
        assert_eq!(topup_request(1, [4; 32], bad), Err(NO_AMOUNT), "{bad}");
    }
}

#[test]
fn a_claim_is_one_hash_and_nothing_waiting_says_so() {
    assert!(
        matches!(outcome(JobKind::Claim, Ok(&[2; 32])), Outcome::Sent(l) if l.starts_with("claim sent: 0x02020202"))
    );
    assert!(
        matches!(outcome(JobKind::Claim, Ok(&[2; 64])), Outcome::Failed(_)),
        "a claim is one hash"
    );
    match outcome(JobKind::Claim, Err(ENOENT)) {
        Outcome::Failed(why) => assert!(why.contains("Nothing waits to be claimed")),
        other => panic!("{other:?}"),
    }
}
