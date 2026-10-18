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

use nonos_settle_proto::{Settled, OP_SETTLE, SETTLED_MAX};

#[test]
fn a_round_names_its_transaction_and_what_it_carries() {
    assert_eq!(OP_SETTLE, 10);
    let s = Settled { tx_hash: [9; 32], carried: vec![([1; 32], 500), ([2; 32], 7)] };
    let b = s.encode().unwrap();
    assert_eq!(b.len(), 33 + 2 * 48);
    assert_eq!(Settled::parse(&b), Some(s.clone()));
    for n in 0..b.len() {
        assert_eq!(Settled::parse(&b[..n]), None, "{n}");
    }
    assert_eq!(Settled::parse(&[b.as_slice(), &[0]].concat()), None, "a tail");
    assert!(s.carries(&[1; 32], 500) && s.carries(&[1; 32], 499));
    assert!(!s.carries(&[1; 32], 501), "an older settle does not carry a newer voucher");
    assert!(!s.carries(&[3; 32], 1), "another channel");
    let none = Settled { tx_hash: [9; 32], carried: vec![] }.encode().unwrap();
    assert_eq!(Settled::parse(&none), None, "a round carries something or is ENOENT");
    let many = Settled { tx_hash: [9; 32], carried: vec![([1; 32], 1); SETTLED_MAX + 1] };
    assert_eq!(many.encode(), None, "one past SETTLED_MAX is refused, not written");
}
