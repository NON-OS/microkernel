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

//! The syscall tables, held to the enum after they were split into parts:
//! the registry names every SyscallNumber exactly once, by its own tag and
//! its own name, which is also what Debug prints ([CAP-DENY] lines); and the
//! SYS_* constants, kept in numbers/ parts, repeat no name and no tag.

use std::collections::BTreeSet;

use super::abi::{lookup_id, REGISTRY};

const DEFS: &str = include_str!("../../../../src/syscall/numbers/defs.rs");
const NUMBERS: &str = concat!(
    include_str!("../../../../src/syscall/microkernel/numbers/acpi.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/data.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/foreign.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/hardware.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/ipc_process.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/session.rs"),
    include_str!("../../../../src/syscall/microkernel/numbers/trust.rs"),
);

/// Every variant and its tag, as defs.rs declares them.
fn variants() -> Vec<(&'static str, [u8; 4])> {
    DEFS.lines()
        .filter_map(|l| {
            let (name, rest) = l.trim().split_once(" = ")?;
            let at = rest.find("tag4(b\"")? + 7;
            Some((name, rest.as_bytes()[at..at + 4].try_into().ok()?))
        })
        .collect()
}

#[test]
fn the_registry_names_every_variant_once_by_its_tag_and_name() {
    let all = variants();
    assert!(all.len() > 150, "defs.rs read: {} variants", all.len());
    let entries: Vec<_> = REGISTRY.iter().flat_map(|s| s.iter()).collect();
    assert_eq!(entries.len(), all.len(), "one entry per variant");
    let ids: BTreeSet<u64> = entries.iter().map(|e| e.id).collect();
    assert_eq!(ids.len(), entries.len(), "no tag twice");
    for (name, tag) in all {
        let id = u64::from(u32::from_le_bytes(tag));
        let v = lookup_id(id).unwrap_or_else(|| panic!("{name} is not in the registry"));
        assert_eq!((v as u64, format!("{v:?}")), (id, name.to_string()));
        assert_eq!(entries.iter().filter(|e| e.name == name && e.id == id).count(), 1, "{name}");
    }
}

#[test]
fn the_sys_constants_repeat_no_name_and_no_tag() {
    let consts: Vec<(&str, &str)> = NUMBERS
        .lines()
        .filter_map(|l| l.strip_prefix("pub const "))
        .filter_map(|l| Some((l.split(':').next()?, l.split("b\"").nth(1)?.get(..4)?)))
        .collect();
    assert!(consts.len() > 150, "{} constants", consts.len());
    let names: BTreeSet<_> = consts.iter().map(|c| c.0).collect();
    let tags: BTreeSet<_> = consts.iter().map(|c| c.1).collect();
    assert_eq!((names.len(), tags.len()), (consts.len(), consts.len()));
}
