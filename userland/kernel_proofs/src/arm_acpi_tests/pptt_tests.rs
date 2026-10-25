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

//! PPTT: a package of two cores, each with a private L1 whose next level
//! is a shared L2; then parents, resources and next levels that name the
//! wrong thing, or loop.

use super::build::*;
use super::pptt_build::*;
use crate::arm_acpi::pptt::pptt;
use crate::arm_acpi::sdt::TableError;

#[test]
fn a_package_of_two_cores() {
    let p = pptt(&table()).unwrap();
    assert_eq!(p.nodes.len(), 3);
    assert_eq!((p.nodes[0].offset, p.nodes[0].parent, p.nodes[0].flags & 1), (64, None, 1));
    assert_eq!(
        (p.nodes[1].parent, p.nodes[1].acpi_id, p.nodes[1].resources.as_slice()),
        (Some(64), Some(0), &[84][..])
    );
    assert_eq!(p.nodes[2].acpi_id, Some(1));
    assert_eq!(
        (p.caches[1].next_level, p.caches[1].size, p.caches[1].id),
        (Some(36), 64 << 10, Some(10))
    );
    assert_eq!((p.caches[0].ways, p.caches[0].line), (4, 64));
}

fn with(parts: &[Vec<u8>]) -> Result<crate::arm_acpi::pptt::Pptt, TableError> {
    pptt(&sdt(b"PPTT", 3, &parts.concat()))
}

#[test]
fn references_that_name_the_wrong_thing_or_loop() {
    assert_eq!(with(&[node(0, 99, 0, &[])]), Err(TableError::Reference(36)), "no such parent");
    assert_eq!(
        with(&[cache(0, 1, 0), node(0, 36, 0, &[])]),
        Err(TableError::Reference(64)),
        "a cache as parent"
    );
    assert_eq!(
        with(&[node(0, 0, 0, &[7])]),
        Err(TableError::Reference(36)),
        "a resource naming nothing"
    );
    assert_eq!(
        with(&[cache(36, 1, 0)]),
        Err(TableError::Reference(36)),
        "a cache its own next level"
    );
    let looped = with(&[node(0, 56, 0, &[]), node(0, 36, 1, &[])]);
    assert!(matches!(looped, Err(TableError::Reference(_))), "two nodes each the other's parent");
    let mut many = node(0, 0, 0, &[]);
    many[16..20].copy_from_slice(&u32le(1000));
    assert_eq!(with(&[many]), Err(TableError::Structure(36)), "more resources than the node holds");
}
