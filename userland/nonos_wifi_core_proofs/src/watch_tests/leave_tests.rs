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

//! Deauthentication and disassociation. Without management frame protection
//! they end the link as they come; with it, one in the clear is never taken,
//! a protected one is (once: a replay is dropped), and reason 6 or 7 in the
//! clear (an access point that rebooted) starts an SA Query.

use super::frames::{mgmt, protected, sa_query_of, ACTION, DEAUTH, DISASSOC};
use super::sim::{joined, STA};
use nonos_wifi_core::station::{Loss, Rx, RxDrop, Tick, SA_QUERY_MAX_MS, SA_QUERY_RETRY_MS};

#[test]
fn without_pmf_a_deauthentication_or_disassociation_ends_the_link() {
    for sub in [DEAUTH, DISASSOC] {
        let mut s = joined(false);
        assert!(matches!(s.receive(&mgmt(sub, STA, &[3, 0]), false), Rx::Lost(Loss::Left(3))));
        assert!(!s.is_associated());
    }
}

#[test]
fn a_forged_unprotected_deauthentication_under_pmf_is_ignored_and_queried() {
    let mut s = joined(true);
    s.tick(0);
    for sub in [DEAUTH, DISASSOC] {
        let r = s.receive(&mgmt(sub, STA, &[3, 0]), false);
        assert!(matches!(r, Rx::Dropped(RxDrop::Unprotected)), "reason 3 is not asked about");
    }
    let Rx::Send(req) = s.receive(&mgmt(DEAUTH, STA, &[7, 0]), false) else {
        panic!("no SA Query")
    };
    let q = sa_query_of(&req).expect("an SA Query under the pairwise key");
    assert!(!q.response);
    let again = s.receive(&mgmt(DISASSOC, STA, &[6, 0]), false);
    assert!(matches!(again, Rx::Dropped(RxDrop::Unprotected)), "one query at a time");
    let answer = protected(&mgmt(ACTION, STA, &[8, 1, q.id as u8, (q.id >> 8) as u8]), 3);
    assert!(matches!(s.receive(&answer, false), Rx::Dropped(RxDrop::Management)));
    for t in (100..=3 * SA_QUERY_MAX_MS).step_by(100) {
        assert!(!matches!(s.tick(t), Tick::Lost(_)), "answered at {t}");
    }
    assert!(s.is_associated());
}

#[test]
fn an_sa_query_unanswered_ends_the_link() {
    let mut s = joined(true);
    s.tick(0);
    let Rx::Send(first) = s.receive(&mgmt(DEAUTH, STA, &[7, 0]), false) else { panic!() };
    let mut ids = vec![(0, sa_query_of(&first).unwrap().id)];
    for t in 1..SA_QUERY_MAX_MS {
        match s.tick(t) {
            Tick::Send(f) => ids.push((t, sa_query_of(&f).unwrap().id)),
            other => assert_eq!(other, Tick::Quiet, "at {t}"),
        }
    }
    assert_eq!(s.tick(SA_QUERY_MAX_MS), Tick::Lost(Loss::Unanswered(7)));
    let times: Vec<u64> = ids.iter().map(|&(t, _)| t).collect();
    assert_eq!(times, [0, 1, 2, 3, 4].map(|n| n * SA_QUERY_RETRY_MS), "each retry timeout");
    assert!(ids.windows(2).all(|w| w[0].1 != w[1].1), "each with a new identifier");
    assert!(!s.is_associated());
}
