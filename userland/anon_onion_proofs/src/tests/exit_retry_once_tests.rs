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

//! Which reasons send the open again and which are passed back, and that a
//! second END is passed back whatever its reason, each exit remembered.

use super::exit_far::{BEGIN, END};
use super::exit_retry_tests::opened;

#[test]
fn the_exits_reasons_retry_and_the_destinations_are_passed_back() {
    for reason in 1..=14u8 {
        let (mut n, id, first, other) = opened();
        n.look();
        n.answer(first, END, id, &[reason]);
        let retried = matches!(reason, 4 | 7 | 8 | 9 | 11);
        let want = if retried {
            (other, "opening".into())
        } else {
            (first, std::format!("ended {}", reason))
        };
        assert_eq!(n.stream(id), want, "reason {reason}");
        assert_eq!(n.look().iter().any(|s| s.1 == BEGIN), retried, "reason {reason}");
    }
}

#[test]
fn a_second_end_is_passed_back_and_each_exit_remembered() {
    let (mut n, id, first, other) = opened();
    n.answer(first, END, id, &[4]);
    n.look();
    n.answer(other, END, id, &[9]);
    assert_eq!(n.stream(id), (other, "ended 9".into()), "once only");
    assert!(n.look().is_empty(), "no third BEGIN");
    for c in [first, other] {
        let exit = n.state.circuits.iter().find(|k| k.id == c).unwrap().path[2].rsa_identity;
        assert!(n.state.exits.refused.holds(&exit, 18081));
    }
}
