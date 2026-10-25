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

//! The DRHD QEMU 11 builds for q35 with intel-iommu and an igb (82576): no
//! INCLUDE_PCI_ALL, one IOAPIC scope, then an endpoint for every function,
//! the card's eight SR-IOV VFs among them: 18 in all, past the 16 scopes the
//! parser once kept, so 00:1f.2 (AHCI) and 00:1f.3 (SMBus) were refused.
//! Bytes as the kernel read them from the table, in a development boot.

use super::dmar_scope_count::pci_scopes_named;
use super::scope::{scope_covers, translated_by_some, Cover};
use super::tests::{bridges, parse};

const QEMU_IGB_DRHD: &str = concat!(
    "0000a800000000000000d9fe00000000",
    "0308000000ff0000",
    "0108000000000000",
    "0108000000000100",
    "0108000000000200",
    "0108000000000300",
    "0108000000000400",
    "0108000000000500",
    "0108000000000600",
    "0108000000001500",
    "0108000000001502",
    "0108000000001504",
    "0108000000001506",
    "0108000000001600",
    "0108000000001602",
    "0108000000001604",
    "0108000000001606",
    "0108000000001f00",
    "0108000000001f02",
    "0108000000001f03",
);

fn drhd() -> Vec<u8> {
    let h = QEMU_IGB_DRHD;
    (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn the_fixture_is_the_structure_qemu_gave() {
    let b = drhd();
    assert_eq!(b.len(), 168);
    assert_eq!(u16::from_le_bytes([b[2], b[3]]), 168);
}

#[test]
fn every_function_qemu_names_is_kept_and_covered() {
    let u = parse(&drhd());
    assert!(!u.include_all && !u.truncated);
    assert_eq!((u.count, pci_scopes_named(&drhd(), 16)), (18, 18));
    for bdf in [(0, 0x1f, 2), (0, 0x1f, 3), (0, 0x16, 6), (0, 5, 0), (0, 0, 0)] {
        assert_eq!(scope_covers(&u, 0, bdf, &bridges), Cover::Yes, "{bdf:?}");
        assert!(translated_by_some(&[u], 0, bdf, &bridges), "{bdf:?}");
    }
    assert!(!translated_by_some(&[u], 0, (0, 0x1f, 4), &bridges), "a function it does not name");
}
