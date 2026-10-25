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

//! The bound is applied by the loader itself: `load_capsule_from_vfs`, the
//! one path `MkCapsuleLoad` reaches, calls it before the verified spawn and
//! with the same requested set the spawn is given.

const SPAWN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs"
);

/// The loader's code with every comment removed, line and block alike.
fn code() -> String {
    let src = std::fs::read_to_string(SPAWN).expect("the loader's source");
    let mut out = String::new();
    let mut rest = src.as_str();
    while let Some(at) = rest.find("/*") {
        out.push_str(&rest[..at]);
        let end = rest[at..].find("*/").expect("a closed block comment");
        rest = &rest[at + end + 2..];
    }
    out.push_str(rest);
    out.lines().map(|l| l.split("//").next().unwrap_or("")).collect()
}

#[test]
fn the_loader_checks_the_bound_before_it_spawns() {
    let code: String = code().split_whitespace().collect();
    let at = |s: &str| code.find(s).unwrap_or_else(|| panic!("the loader lost `{s}`"));
    let names = at("super::names::check(service_name,reply_name,");
    let asked = at("letrequested=super::asked::within_manifest(&manifest,requested_caps,service_name)?;");
    let check = at("super::bound::check(&manifest,requested,on_behalf_of,service_name)?;");
    let spawn = at("spawn_verified_as(");
    assert!(names < asked && asked < check, "the manifest check sits between the names and the bound");
    assert!(names < check, "the bound is checked after the names, so its line names a real one");
    assert!(check < spawn, "the bound is checked before the spawn");
    assert!(code.contains("requested_caps:requested,"), "the spawn gets the checked set");
    assert_eq!(code.matches("spawn_verified_as(").count(), 1, "one spawn, after the check");
    assert_eq!(code.matches("bound::check(").count(), 1, "one check");
    assert_eq!(code.matches("requested_caps&").count(), 0, "no silent narrowing of the request");
}
