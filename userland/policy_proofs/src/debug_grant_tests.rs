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

//! The policy capsule writes its "[POLICY]" lines with MkDebug, which only a
//! holder of Debug may call. Its manifest offers Debug and the kernel asks
//! for it on a serial-debug build, and it holds no secret for the kernel to
//! withhold Debug over: the lines a boot found refused (a work area,
//! finding 4) reach the log.

const MK: &str = include_str!("../../capsule_policy/Capsule.mk");
const SPAWN: &str = include_str!("../../../src/userspace/capsule_policy/spawn.rs");
const SECRET_LOG: &str =
    include_str!("../../../src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs");
const DEBUG: u64 = 1 << 8;

fn mask(name: &str) -> u64 {
    let line = MK.lines().find(|l| l.starts_with(name)).unwrap_or_else(|| panic!("{name} is gone"));
    let hex = line.rsplit(":=").next().map(str::trim).and_then(|v| v.strip_prefix("0x"));
    u64::from_str_radix(hex.unwrap_or_else(|| panic!("{line}")), 16).unwrap()
}

/// The bits secret_log withholds Debug over, read from the kernel's file.
fn secrets() -> u64 {
    let line = SECRET_LOG.lines().find(|l| l.contains("const SECRETS")).expect("SECRETS");
    let expr = line.split('=').nth(1).expect("its value").trim_end_matches(';');
    expr.split('|')
        .map(|t| {
            let shift = t.trim().strip_prefix("1 <<").expect("a 1 << n term").trim();
            1u64 << shift.parse::<u32>().unwrap()
        })
        .fold(0, |a, b| a | b)
}

fn uses_mk_debug(dir: &std::path::Path) -> bool {
    std::fs::read_dir(dir).unwrap().flatten().any(|e| {
        let p = e.path();
        if p.is_dir() {
            return uses_mk_debug(&p);
        }
        p.extension().is_some_and(|x| x == "rs")
            && std::fs::read_to_string(&p).unwrap().contains("mk_debug(")
    })
}

#[test]
fn the_manifest_offers_debug_and_requires_no_more() {
    // CoreExec, IPC, Memory, FileSystem and PolicyPush (bit 42): the Admin
    // split took Admin away and left policy its one kernel call.
    assert_eq!(mask("CAPSULE_REQUIRED_CAPS"), 0x400_0000_0059);
    assert_eq!(mask("CAPSULE_OPTIONAL_CAPS"), DEBUG);
    assert_eq!(mask("CAPSULE_REQUIRED_CAPS") & DEBUG, 0, "Debug stays optional");
}

#[test]
fn the_kernel_asks_for_debug_on_a_serial_debug_build() {
    let granted = SPAWN.find("const GRANTED_CAPS").map(|at| &SPAWN[at..]).expect("GRANTED_CAPS");
    let granted = &granted[..granted.find(';').unwrap()];
    assert!(granted.contains("REQUIRED_CAPS | crate::capabilities::serial_debug_cap()"));
    assert!(SPAWN.contains("requested_caps: GRANTED_CAPS,"));
    assert!(SPAWN.contains("const REQUIRED_CAPS: u64 = Capability::CoreExec.bit()"));
    assert!(SPAWN.contains("| Capability::PolicyPush.bit();"));
    assert!(!SPAWN.contains("Capability::Admin"), "policy holds no Admin since the split");
}

#[test]
fn policy_holds_no_secret_so_debug_is_not_withheld() {
    let s = secrets();
    assert_eq!(s.count_ones(), 4, "{s:#x}");
    assert_eq!((mask("CAPSULE_REQUIRED_CAPS") | mask("CAPSULE_OPTIONAL_CAPS")) & s, 0);
}

#[test]
fn the_capsule_still_logs_through_mk_debug() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_policy/src");
    assert!(uses_mk_debug(&src), "the grant is for these lines");
}
