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

//! The crate's capability names are the kernel's, each at its own bit, read
//! from the kernel source itself; and the nine forbidden ones are among them.

use nsh_roles::caps::{bit, forbidden_mask, FORBIDDEN, KERNEL_CAPS};
use nsh_roles::CapSet;

/// Every `Name = 1 << n,` line of the kernel's capability table.
fn kernel_table() -> Vec<(String, u32)> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../src/capabilities/types/defs.rs");
    let src = std::fs::read_to_string(path).expect("the kernel's capability table");
    let parse = |l: &str| {
        let (name, rest) = l.trim().split_once(" = 1 << ")?;
        // A one-line entry carries its doc as an attribute in front.
        let name = name.rsplit_once(' ').map_or(name, |(_, n)| n);
        Some((name.to_string(), rest.trim_end_matches(',').parse().ok()?))
    };
    src.lines().filter_map(parse).collect()
}

#[test]
fn every_name_is_the_kernels_at_its_bit() {
    let table = kernel_table();
    assert_eq!(table.len(), KERNEL_CAPS.len());
    assert_eq!(KERNEL_CAPS.len(), nsh_wire::MAX_CAPS);
    for (name, b) in &table {
        assert_eq!(KERNEL_CAPS[*b as usize], name, "bit {b}");
        assert_eq!(bit(name.as_bytes()), Some(*b));
    }
}

#[test]
fn the_forbidden_nine_are_kernel_names() {
    assert!(FORBIDDEN.iter().all(|n| bit(n.as_bytes()).is_some()));
    assert_eq!(forbidden_mask().count_ones(), 9);
}

#[test]
fn a_name_the_kernel_lacks_has_no_bit() {
    for name in [&b"Root"[..], b"admin", b"CoreExe", b"CoreExecX", b""] {
        assert_eq!(bit(name), None);
    }
}

#[test]
fn a_set_lists_its_names_in_ascending_byte_order() {
    let mut out = Vec::new();
    crate::build::caps(&mut out, &["Network", "IPC", "CoreExec", "AttestRead"]);
    let names = nsh_wire::CapNames::read(&mut nsh_wire::Reader::new(&out)).expect("names");
    let set = CapSet::named(&names).expect("known");
    assert_eq!(set.names(), ["AttestRead", "CoreExec", "IPC", "Network"]);
    assert!(set.contains(b"IPC") && !set.contains(b"Memory") && !set.contains(b"Bogus"));
    assert_eq!(set.bits().count_ones(), 4);
}
