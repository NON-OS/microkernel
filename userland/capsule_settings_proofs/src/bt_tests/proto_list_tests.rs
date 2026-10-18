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

//! BONDS's and BATTERY's lists: the count, the bound and the length agree,
//! and every octet is one v1 defines.

use super::fixture::{bond, list, KB};
use nonos_bt_proto::{Bonds, Level, Levels, BONDS_MAX, LEVELS_MAX};

#[test]
fn a_bond_list_agrees_with_its_count() {
    let one = list(&[bond(1, KB, true, false)]);
    let b = Bonds::read(&one).unwrap();
    assert_eq!((b.len(), b.get(0).unwrap().addr, b.get(0).unwrap().kind), (1, KB, 1));
    assert!(b.get(0).unwrap().authenticated && !b.get(0).unwrap().linked);
    assert!(Bonds::read(&[0]).unwrap().is_empty());
    assert!(Bonds::read(&[]).is_none());
    assert!(Bonds::read(&one[..one.len() - 1]).is_none(), "an entry cut short");
    assert!(Bonds::read(&[one.clone(), vec![0]].concat()).is_none(), "an octet over");
    for (at, what) in [(1, "address type"), (8, "authenticated"), (9, "linked")] {
        let mut bad = one.clone();
        bad[at] = 2;
        assert!(Bonds::read(&bad).is_none(), "{what}");
    }
    let full: Vec<_> = (0..=BONDS_MAX as u8).map(|i| bond(0, [i; 6], false, false)).collect();
    assert!(Bonds::read(&list(&full[..BONDS_MAX])).is_some());
    assert!(Bonds::read(&list(&full)).is_none(), "more bonds than the capsule keeps");
}

#[test]
fn a_level_is_a_percent() {
    let l = Levels::read(&[2, 0, 1, 100, 0xFF, 0, 0]).unwrap();
    assert_eq!(l.get(0), Some(Level { place: 0, instance: 1, percent: 100 }));
    assert_eq!(l.get(1), Some(Level { place: 0xFF, instance: 0, percent: 0 }));
    assert_eq!(l.get(2), None);
    assert!(Levels::read(&[1, 0, 0, 101]).is_none());
    assert!(Levels::read(&[2, 0, 0, 50]).is_none());
    let over = [vec![LEVELS_MAX as u8 + 1], vec![0; 3 * (LEVELS_MAX + 1)]].concat();
    assert!(Levels::read(&over).is_none());
}
