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

//! ETH beside NOX (v1.4.55, v1.4.56): read by GAS after balanceOf for the
//! same address; money at an address is NOX or ETH, and an address is read
//! as holding nothing only when both reads answered.

use crate::shield::due::{Due, Why};
use crate::shield::landed_held::{both, Held};
use crate::shield::scan::Scan;

fn batch(from: u32) -> Vec<(u32, [u8; 20])> {
    (from..from + 64).map(|i| (i, [i as u8; 20])).collect()
}

#[test]
fn eth_alone_is_a_hit_and_a_half_answer_is_no_answer() {
    let mut s = Scan::default();
    s.start();
    s.batch(batch(0));
    s.read(both(Some(0), Some(1)));
    assert_eq!(s.found, [(0, [0; 20], Held { nox: 0, eth: 1 })], "gas money left there");
    s.read(both(Some(5), None));
    assert_eq!((s.at, s.tries, s.found.len()), (1, 1, 1), "ETH unread: tried again");
}

#[test]
fn eth_left_keeps_an_address_due_until_both_are_gone() {
    let mut d = Due::default();
    d.add([3; 20], Why::RefundSent, 0);
    d.read([3; 20], both(Some(4), Some(2)), 1);
    d.read([3; 20], both(Some(0), Some(2)), 2);
    assert_eq!(d.entries[0].held, both(Some(0), Some(2)), "the gas money is still there");
    d.read([3; 20], both(Some(0), None), 3);
    assert_eq!(d.entries.len(), 1, "a half answer is no answer");
    d.read([3; 20], both(Some(0), Some(0)), 4);
    assert!(d.entries.is_empty());
}
