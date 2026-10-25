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

//! StreamImport in the Linux personality: the install role alone asks for
//! it, so `linux install` may start the model fetcher within its own
//! capabilities, and no other role, the runs that host guests among them,
//! is ever granted it.

use super::source::code_of;
use crate::stand_in::world;
use crate::tool_bound::check;

const STREAM_IMPORT: u64 = 1 << 34;
const LINUX: &str = include_str!("../../../capsule_linux/Capsule.mk");
const FETCHER: &str = include_str!("../../../capsule_model_fetch/Capsule.mk");

fn field(mk: &str, key: &str) -> u64 {
    let line = mk.lines().find(|l| l.starts_with(key)).expect("the field");
    let hex = line.rsplit(":=").next().unwrap().trim().trim_start_matches("0x");
    u64::from_str_radix(hex, 16).expect("a hex mask")
}

/// What a role installs, as the verified spawn grants it.
fn role(extra: u64) -> u64 {
    let base = code_of(include_str!("../../../../src/userspace/capsule_linux/spawn.rs"));
    assert!(!base.contains("StreamImport"), "LINUX_CAPS never asks for it");
    field(LINUX, "CAPSULE_REQUIRED_CAPS") | (field(LINUX, "CAPSULE_OPTIONAL_CAPS") & extra)
}

#[test]
fn only_the_install_role_asks_for_stream_import() {
    assert_eq!(field(LINUX, "CAPSULE_REQUIRED_CAPS") & STREAM_IMPORT, 0, "never required");
    assert_ne!(field(LINUX, "CAPSULE_OPTIONAL_CAPS") & STREAM_IMPORT, 0);
    let roles = code_of(include_str!("../../../../src/userspace/capsule_linux/roles.rs"));
    let flat: String = roles.split_whitespace().collect();
    let at = flat.find("constINSTALL:Role").expect("the install role");
    let next = flat[at..].find("};").expect("its end") + at;
    assert_eq!(flat.matches("StreamImport").count(), 1, "named once in roles.rs");
    let named = flat.find("StreamImport").unwrap();
    assert!(at < named && named < next, "and that once, in INSTALL");
    for other in [
        include_str!("../../../../src/userspace/capsule_linux/terminal/roles.rs"),
        include_str!("../../../../src/userspace/capsule_linux/run_net.rs"),
        include_str!("../../../../src/userspace/capsule_linux/role_spawn.rs"),
    ] {
        assert!(!code_of(other).contains("StreamImport"), "no other role asks");
    }
}

#[test]
fn the_install_role_starts_the_fetcher_and_no_other_role_does() {
    let fetcher = field(FETCHER, "CAPSULE_REQUIRED_CAPS");
    assert_ne!(fetcher & STREAM_IMPORT, 0);
    let manifest = [fetcher.to_be_bytes(), 0u64.to_be_bytes()].concat();
    world(Some(20), &[(20, role((1 << 2) | (1 << 33) | STREAM_IMPORT))]);
    assert_eq!(check(&manifest, fetcher, b"tool.model-fetch"), Ok(()));
    for extra in [0, 1 << 2] {
        world(Some(21), &[(21, role(extra))]);
        assert_eq!(check(&manifest, fetcher, b"tool.model-fetch"), Err(-1), "a run is refused");
    }
}
