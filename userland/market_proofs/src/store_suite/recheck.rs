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

//! While the market reads a sold listing's bond (v1.4.23), the store says
//! so, and asks the gates again when the recheck is due, never sooner; a
//! listing blocked by any other gate is Blocked, not Checking.

use nonos_market_proto::{BondCode, Readiness};

use crate::store::fill_order::{next, recheck, RECHECKS, RECHECK_MS};
use crate::store::listing::Listing;
use crate::store::verdict::{Verdict, BOND_GATE};

fn gates(failing: &[usize]) -> Readiness {
    let mut gates = [true; 11];
    failing.iter().for_each(|i| gates[*i] = false);
    Readiness { install_ready: failing.is_empty(), gates, bond: BondCode::Unread }
}

#[test]
fn only_the_bond_unread_is_checking() {
    assert!(Verdict::of(&gates(&[])) == Verdict::Ready);
    assert!(Verdict::of(&gates(&[BOND_GATE])) == Verdict::Checking);
    assert!(Verdict::of(&gates(&[1])) == Verdict::Installed);
    assert!(Verdict::of(&gates(&[BOND_GATE, 6])) == Verdict::Blocked, "commerce fails too");
    assert!(Verdict::of(&gates(&[1, BOND_GATE])) == Verdict::Blocked);
    let read = Readiness { bond: BondCode::Inactive, ..gates(&[BOND_GATE]) };
    assert!(Verdict::of(&read) == Verdict::Blocked, "a bond read and failed is final");
    assert_eq!(Verdict::Checking.sentence(), b"Checking the seller's bond on chain");
}

#[test]
fn a_recheck_is_asked_when_due_and_not_before() {
    let mut l = Listing::new(b"community.x".to_vec(), [0; 32], b"X".to_vec(), false);
    l.known.described = true;
    l.known.judged = true;
    l.known.recheck_at = Some(10_000);
    let all = vec![l];
    assert_eq!(next(&all, Some(0), 9_999), None, "not yet");
    assert_eq!(next(&all, Some(0), 10_000), Some((0, true)));
    assert_eq!(next(&all, None, 10_000), None, "only the selection is rechecked");
    assert!(RECHECKS as u64 * RECHECK_MS <= 60_000, "a minute of asking at most");
}

#[test]
fn rechecks_stop_at_their_cap_or_once_the_gates_settle() {
    let mut l = Listing::new(b"community.x".to_vec(), [0; 32], b"X".to_vec(), false);
    l.known.readiness = Some(gates(&[BOND_GATE]));
    for n in 1..=RECHECKS {
        recheck(&mut l.known, 0);
        assert_eq!((l.known.recheck_at, l.known.rechecks), (Some(RECHECK_MS), n));
    }
    recheck(&mut l.known, 0);
    assert_eq!(l.known.recheck_at, None, "past the cap the gates stand as they are");
    l.known.rechecks = 0;
    l.known.readiness = Some(gates(&[]));
    recheck(&mut l.known, 0);
    assert_eq!((l.known.recheck_at, l.known.rechecks), (None, 0), "ready: nothing to recheck");
}
