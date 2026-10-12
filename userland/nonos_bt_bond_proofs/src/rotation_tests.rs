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

//! This host's private address renewed every TGAP(private_addr_int), 15
//! minutes (Vol 3, Part C, Appendix A), each one resolving under its IRK.

use nonos_bt_bond::{LocalIrk, Rotation, ROTATE_MS};
use nonos_bt_smp::privacy::resolves;
use nonos_bt_wire::addr::RandomKind;

const KEY: [u8; 16] = *b"this-hosts-irk!!";

#[test]
fn an_address_is_due_at_once_and_then_every_fifteen_minutes() {
    let mut r = Rotation::new(LocalIrk::new(KEY).unwrap());
    assert_eq!(ROTATE_MS, 900_000);
    assert!(r.due(0));
    let first = r.renew([0x12, 0x34, 0x56], 1_000).unwrap();
    assert_eq!(r.current(), Some(first));
    assert!(!r.due(1_000 + ROTATE_MS - 1));
    assert!(r.due(1_000 + ROTATE_MS));
    // A clock that went back does not hold the address forever.
    assert!(!r.due(0));
    let second = r.renew([0x65, 0x43, 0x21], 1_000 + ROTATE_MS).unwrap();
    assert_ne!(first, second);
    for a in [first, second] {
        assert_eq!(a.random_kind(), Ok(RandomKind::Resolvable));
        assert!(resolves(r.irk().key(), &a));
    }
}

#[test]
fn a_forbidden_draw_makes_nothing_and_keeps_the_address() {
    let mut r = Rotation::new(LocalIrk::new(KEY).unwrap());
    let a = r.renew([1, 2, 3], 5).unwrap();
    assert_eq!(r.renew([0, 0, 0], 10), None);
    assert_eq!(r.renew([0xff, 0xff, 0xff], 10), None);
    assert_eq!(r.current(), Some(a));
    assert!(!r.due(10));
}
