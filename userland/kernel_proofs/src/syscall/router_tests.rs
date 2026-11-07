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

//! The router's hot arms (`dispatch_fn.rs`, first match) send each syscall
//! to the family the general match would. The general match tries crypto,
//! admin, microkernel ops and graphics before surface and input, so a hot
//! surface or input number must be in none of those, and a hot IPC or futex
//! number must be a microkernel op and in none before it.

use super::numbers::SyscallNumber;

#[path = "../../../../src/syscall/dispatch/router/microkernel_ops"]
mod microkernel_ops {
    mod ops1;
    mod ops2;
    mod ops3;
    mod ops4;

    pub fn matches(nr: super::SyscallNumber) -> bool {
        ops1::matches(nr) || ops2::matches(nr) || ops3::matches(nr) || ops4::matches(nr)
    }
}

const ROUTER: &str = include_str!("../../../../src/syscall/dispatch/router/dispatch_fn.rs");

/// The variant names in the arm of the hot match that calls `target`.
fn hot_arm(target: &str) -> Vec<String> {
    let hot = ROUTER.split("_ => {}").next().unwrap_or("");
    let parts: Vec<&str> = hot.split("=> {").collect();
    let call = format!("return {target}");
    let i = (1..parts.len())
        .find(|&i| parts[i].trim_start().starts_with(&call))
        .unwrap_or_else(|| panic!("no hot arm for {target}"));
    let pattern = parts[i - 1].rsplit('}').next().unwrap_or("");
    pattern
        .split("SyscallNumber::")
        .skip(1)
        .map(|v| v.chars().take_while(|c| c.is_alphanumeric()).collect())
        .collect()
}

fn named(name: &str) -> SyscallNumber {
    super::abi::REGISTRY
        .iter()
        .flat_map(|s| s.iter())
        .find(|e| e.name == name)
        .map(|e| e.variant)
        .unwrap_or_else(|| panic!("{name} is not registered"))
}

fn before_surface(nr: SyscallNumber) -> bool {
    let name = format!("{nr:?}");
    name.starts_with("Crypto")
        || name.starts_with("Admin")
        || name.starts_with("Graphics")
        || microkernel_ops::matches(nr)
}

#[test]
fn hot_surface_and_input_numbers_are_claimed_by_nothing_earlier() {
    let surface = hot_arm("surface_ops::handle");
    let input = hot_arm("input_ops::handle");
    assert_eq!(surface, ["MkSurfacePresent", "MkDisplayVsyncWait"]);
    assert_eq!(input, ["MkInputEventDrain", "MkInputEventWait"]);
    for name in surface.iter().chain(input.iter()) {
        assert!(!before_surface(named(name)), "{name} is routed before the surface arm");
    }
}

#[test]
fn hot_ipc_and_futex_numbers_are_microkernel_ops() {
    let ops = hot_arm("microkernel_ops::handle");
    assert_eq!(ops.len(), 10, "{ops:?}");
    for name in &ops {
        let nr = named(name);
        assert!(microkernel_ops::matches(nr), "{name}");
        let n = format!("{nr:?}");
        assert!(!n.starts_with("Crypto") && !n.starts_with("Admin"), "{name}");
    }
}
