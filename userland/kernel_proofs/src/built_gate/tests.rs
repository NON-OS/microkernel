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

use super::rule::gate;

#[derive(Debug, PartialEq, Clone, Copy)]
enum E {
    NotBuilt,
}

#[test]
fn an_empty_image_is_not_built_and_any_other_is_let_through() {
    assert_eq!(gate(&[], E::NotBuilt), Err(E::NotBuilt));
    assert_eq!(gate(&[0x7F], E::NotBuilt), Ok(()));
}

/// The store load asks `refused` (empty.rs, the gate above) after the bound
/// and before it spawns, so an empty stored image never reaches preflight.
#[test]
fn the_store_load_refuses_an_empty_image_before_it_spawns() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs"
    ))
    .expect("the store load's source");
    let at = |s: &str| src.find(s).unwrap_or_else(|| panic!("the store load lost `{s}`"));
    let bound = at("super::bound::check(");
    let refused = at("super::empty::refused(&artifacts.elf, service_name)?;");
    assert!(bound < refused && refused < at("spawn_verified_as("));
}
