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

use std::cell::Cell;

use super::rule::{gate, stored};

#[derive(Debug, PartialEq, Clone, Copy)]
enum E {
    NotBuilt,
    EmptyInStore,
}

#[test]
fn an_empty_image_is_not_built_and_any_other_is_let_through() {
    assert_eq!(gate(&[], E::NotBuilt), Err(E::NotBuilt));
    assert_eq!(gate(&[0x7F], E::NotBuilt), Ok(()));
}

#[test]
fn an_empty_image_in_the_store_is_refused_by_its_own_name() {
    let spawned = Cell::new(false);
    let load = |elf: &[u8]| {
        stored(elf, E::EmptyInStore, || {
            spawned.set(true);
            gate(elf, E::NotBuilt).map(|()| 7)
        })
    };
    assert_eq!(load(&[]), Err(E::EmptyInStore), "never \"not built\"");
    assert!(!spawned.get());
    assert_eq!(load(&[1]), Ok(7));
}
