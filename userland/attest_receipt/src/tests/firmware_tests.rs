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

//! The firmware entry on a receipt: folded with the rest, never listed or
//! counted as a running program.

use super::fixtures::{entry, flatten, kernel_fold, three_capsules, BROWSER};
use crate::{parse_entries, render, verify_root};

/// A registry with the firmware measurement filed under pid 0, first, the
/// way pid order puts it. Its mask is empty: tables reach nothing.
fn with_firmware() -> Vec<[u8; crate::kernel::ENTRY_LEN]> {
    let mut set = vec![entry(0, 0xF0, 0, 0)];
    set.extend(three_capsules());
    set
}

fn rendered(set: &[[u8; crate::kernel::ENTRY_LEN]]) -> String {
    let entries = parse_entries(&flatten(set)).unwrap();
    verify_root(&flatten(set), kernel_fold(set)).unwrap();
    render(&entries)
}

#[test]
fn a_pid_zero_entry_is_never_a_row_of_the_capsule_table() {
    let out = rendered(&with_firmware());
    assert!(!out.lines().any(|l| l.trim_start().starts_with("0 ")), "{out}");
    let fw: Vec<&str> = out.lines().filter(|l| l.contains("before any _INI")).collect();
    assert_eq!(fw, ["    -  f0f0f0f0f0f0     -     firmware tables, measured before any _INI"]);
}

#[test]
fn the_entry_is_folded_with_the_rest_and_counted_apart() {
    let set = with_firmware();
    assert!(verify_root(&flatten(&set), kernel_fold(&three_capsules())).is_err());
    let out = rendered(&set);
    assert!(out.starts_with("3 capsules and the firmware tables, all folded"), "{out}");
    assert!(out.contains("1 of 3 capsules can reach the network. 3 were proved"), "{out}");
}

#[test]
fn without_the_entry_the_receipt_reads_as_before() {
    let out = rendered(&three_capsules());
    assert!(out.starts_with("3 capsules, all folded"), "{out}");
    assert!(!out.contains("firmware"), "{out}");
}

#[test]
fn a_registry_of_only_firmware_lists_no_capsule() {
    let out = rendered(&[entry(0, 0xF0, 0, 0)]);
    assert!(out.starts_with("0 capsules and the firmware tables"), "{out}");
    assert!(out.contains("0 of 0 capsules can reach the network"), "{out}");
    assert!(!out.contains("WARNING"), "{out}");
}

#[test]
fn a_pid_zero_entry_holding_capabilities_is_reported_not_hidden() {
    let out = rendered(&[entry(0, 0xF0, BROWSER, 0)]);
    assert!(out.contains("WARNING: the pid 0 entry carries capabilities: "), "{out}");
    assert!(out.contains("0 of 0 capsules can reach the network"), "{out}");
}
