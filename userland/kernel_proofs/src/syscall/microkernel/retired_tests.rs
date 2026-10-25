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

//! A retired number answers EINVAL to every caller and no other number is
//! retired: MBGL, MkBootLogo, is kept in the published table and refused.

use super::errnos::ERRNO_INVAL;
use super::owner_gate_tests::{tag_of, DISPATCH};
use super::retired::{handle, RETIRED};

const ABI: &str = include_str!("../../../../../abi/syscalls.toml");
const ROUTE: &str = include_str!("../../../../../src/syscall/microkernel/dispatch/route.rs");

fn nr(tag: [u8; 4]) -> u64 {
    u64::from(u32::from_le_bytes(tag))
}

#[test]
fn mkbootlogo_answers_einval() {
    assert_eq!(ERRNO_INVAL, -22);
    assert_eq!(handle(nr(tag_of("SYS_BOOT_LOGO"))), Some(-22));
    assert_eq!(handle(0x4C47_424D), Some(-22), "MBGL as abi/syscalls.toml has it");
}

#[test]
fn no_other_number_is_retired() {
    for name in ["SYS_ACPI_TABLE", "SYS_ACPI_MEASURED", "SYS_NUMA_NODE_OF", "SYS_ACPI_LPIT"] {
        assert_eq!(handle(nr(tag_of(name))), None, "{name}");
    }
    for n in [0, 1, u64::MAX, nr(*b"MBGM"), nr(*b"LBGM"), 0x1_4C47_424D] {
        assert_eq!(handle(n), None, "{n:#x}");
    }
}

#[test]
fn a_retired_number_stays_published_and_unrouted() {
    for t in RETIRED {
        let tag = core::str::from_utf8(&t).unwrap();
        assert!(ABI.contains(&format!("\n{tag} = 0x{:08X}", nr(t))), "{tag} kept");
        assert!(ABI.contains(&format!("[desc.{tag}]\n# Retired")), "{tag} said retired");
    }
    assert!(!DISPATCH.contains("SYS_BOOT_LOGO"), "the ACPI dispatch no longer routes it");
    let route = &ROUTE[ROUTE.find("fn route(").unwrap()..];
    let first = route.find("retired::handle(nr)").expect("the router asks");
    assert!(first < route.find("ipc::handle").unwrap(), "before any dispatch");
}
