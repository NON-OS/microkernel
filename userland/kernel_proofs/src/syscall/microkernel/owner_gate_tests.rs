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

//! F-95: every number the ACPI dispatch routes is in exactly one of the
//! gate's lists, read from the kernel's own source; the capsule's calls,
//! the twelve that once took the Admin bit alone among them, are the
//! owner's; and the shared ones are not.

use super::owner_gate::{owner_only, OWNER_ONLY};

/// Read by others, each under its own gate: device names by enumerators
/// (a holder question only for the owner, see `asker`), USB port and NUMA
/// facts by drivers, the measured tables by attestation readers.
const SHARED: [[u8; 4]; 7] = [*b"MADN", *b"MUPF", *b"MNNO", *b"MNDS", *b"MDNN", *b"MNHM", *b"MAMT"];

pub(super) const DISPATCH: &str =
    include_str!("../../../../../src/syscall/microkernel/acpi/dispatch.rs");
/// The SYS_* constants, kept in parts under numbers/.
pub(super) const NUMBERS: &str = concat!(
    include_str!("../../../../../src/syscall/microkernel/numbers/acpi.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/data.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/foreign.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/hardware.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/ipc_process.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/session.rs"),
    include_str!("../../../../../src/syscall/microkernel/numbers/trust.rs"),
);

pub(super) fn tag_of(name: &str) -> [u8; 4] {
    let line = NUMBERS.lines().find(|l| l.contains(&format!("const {name}: u64"))).expect(name);
    let at = line.find("b\"").expect("a tag") + 2;
    line.as_bytes()[at..at + 4].try_into().expect("four bytes")
}

fn routed() -> Vec<String> {
    let arm = |l: &str| l.trim().split(" =>").next().map(str::to_string);
    DISPATCH
        .lines()
        .filter(|l| l.trim().starts_with("SYS_") && l.contains("=>"))
        .filter_map(arm)
        .collect()
}

#[test]
fn every_routed_number_is_gated_one_way() {
    let names = routed();
    assert_eq!(names.len(), OWNER_ONLY.len() + SHARED.len(), "{names:?}");
    for name in names {
        let t = tag_of(&name);
        let (own, shared) = (OWNER_ONLY.contains(&t), SHARED.contains(&t));
        assert!(own ^ shared, "{name} must be in exactly one list");
        assert_eq!(owner_only(u64::from(u32::from_le_bytes(t))), own, "{name}");
    }
}
