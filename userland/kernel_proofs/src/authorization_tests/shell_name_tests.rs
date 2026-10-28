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
//! The shell the cap table asks after is the one the kernel spawned: the
//! caller owns the endpoint desktop_shell, which the spawn path registers
//! and no capsule may claim at run time, and never a process name. A capsule
//! SERVES pairs with a listing is known the same way, by its baked name.

const CAPABILITY: &str = include_str!("../../../../src/syscall/contract/capability.rs");
const RESERVED: &str = include_str!("../../../../src/services/registry/reserved.rs");
const SPAWN: &str = include_str!("../../../../src/userspace/capsule_desktop_shell/spawn.rs");

#[test]
fn the_shell_is_known_by_the_endpoint_it_was_spawned_under() {
    let gate = CAPABILITY.find("fn caller_launches() -> bool {").expect("the launcher gate");
    let gate = &CAPABILITY[gate..];
    assert!(gate.contains("caller_owns(\"desktop_shell\")"));
    assert!(gate.contains("LISTINGS.iter().any(|&(capsule, _)| caller_owns(capsule))"));
    assert!(CAPABILITY.contains("            caller_launches,\n"));
    assert!(!CAPABILITY.contains("name()"), "a process name is the process's to change");
    assert!(SPAWN.contains("const SERVICE_NAME: &str = \"desktop_shell\";"));
}

#[test]
fn no_capsule_may_register_the_shell_s_name_at_run_time() {
    let start = RESERVED.find("const RUNTIME_REGISTRABLE").expect("the run-time list");
    let list = &RESERVED[start..];
    let list = &list[..list.find(';').expect("its end")];
    assert!(!list.contains("desktop_shell"), "{list}");
}
