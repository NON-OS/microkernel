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

//! The reasons line (B9): the first failed gate named, the bond's said as
//! "checking" while the store still asks and "could not be read" after, and
//! nothing for a listing that is ready.

use nonos_market_proto::Readiness;

use crate::store::reasons::{mark, reason, BOND_UNREAD, CHECKING};
use crate::store::verdict::BOND_GATE;

fn failing(gates: &[usize]) -> Readiness {
    let mut all = [true; 11];
    gates.iter().for_each(|i| all[*i] = false);
    Readiness { install_ready: gates.is_empty(), gates: all }
}

#[test]
fn the_first_failed_gate_is_named() {
    assert_eq!(reason(&failing(&[]), false), None);
    assert_eq!(
        reason(&failing(&[4]), false),
        Some(&b"It is built for another kind of machine"[..])
    );
    assert_eq!(
        reason(&failing(&[6, 4]), false),
        reason(&failing(&[4]), false),
        "the first, not the last"
    );
    for i in 0..10 {
        let said = reason(&failing(&[i]), true).unwrap();
        assert!(!said.is_empty() && said != CHECKING && said != BOND_UNREAD, "gate {i}");
    }
}

#[test]
fn the_bond_is_checking_while_asked_and_unread_after() {
    assert_eq!(reason(&failing(&[BOND_GATE]), true), Some(CHECKING));
    assert_eq!(reason(&failing(&[BOND_GATE]), false), Some(BOND_UNREAD));
    assert_eq!(BOND_UNREAD, b"Seller's bond could not be read");
    assert_eq!(
        (mark(BOND_GATE, false), mark(3, false), mark(3, true)),
        (&b"not checked"[..], &b"fail"[..], &b"pass"[..])
    );
}
