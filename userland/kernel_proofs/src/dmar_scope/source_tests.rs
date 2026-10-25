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

//! The interrupt sources a DRHD scopes (I/O APICs and HPETs), each kept by
//! its own type with the requester id its messages carry (L44).

use super::dmar_namespace_scope::{NamespaceScope, SCOPE_HPET, SCOPE_IOAPIC, SCOPE_NAMESPACE};
use super::dmar_namespace_walk::{walk_namespace_scopes, walk_numbered_scopes};

fn scope(kind: u8, number: u8, bus: u8, hops: &[(u8, u8)]) -> Vec<u8> {
    let mut b = vec![kind, (6 + hops.len() * 2) as u8, 0, 0, number, bus];
    hops.iter().for_each(|(d, f)| b.extend_from_slice(&[*d, *f]));
    b
}

/// A client DRHD with INCLUDE_PCI_ALL: an endpoint, the I/O APIC at
/// 00:1e.7 (APIC id 2), an HPET at 00:1e.6 and a namespace device.
fn client_drhd() -> Vec<u8> {
    let mut drhd = vec![0u8; 16];
    drhd.extend(scope(1, 0, 0, &[(0x02, 0)]));
    drhd.extend(scope(SCOPE_IOAPIC, 2, 0xF0, &[(0x1F, 0)]));
    drhd.extend(scope(SCOPE_HPET, 0, 0x00, &[(0x1E, 6)]));
    drhd.extend(scope(SCOPE_NAMESPACE, 1, 0, &[(0x15, 0)]));
    drhd
}

#[test]
fn each_type_is_kept_apart_with_its_requester_id() {
    let drhd = client_drhd();
    let mut out = [NamespaceScope::EMPTY; 4];
    assert_eq!(walk_numbered_scopes(&drhd, 16, SCOPE_IOAPIC, &mut out), (1, false));
    assert_eq!((out[0].number, out[0].requester()), (2, Some((0xF0, 0x1F, 0))));
    assert_eq!(walk_numbered_scopes(&drhd, 16, SCOPE_HPET, &mut out), (1, false));
    assert_eq!((out[0].number, out[0].requester()), (0, Some((0, 0x1E, 6))));
    assert_eq!(walk_namespace_scopes(&drhd, 16, &mut out), (1, false));
    assert_eq!(out[0].number, 1);
}

#[test]
fn a_full_store_or_a_malformed_scope_says_it_dropped_one() {
    let mut drhd = client_drhd();
    drhd.extend(scope(SCOPE_IOAPIC, 3, 0xF0, &[(0x1F, 1)]));
    let mut one = [NamespaceScope::EMPTY; 1];
    assert_eq!(walk_numbered_scopes(&drhd, 16, SCOPE_IOAPIC, &mut one), (1, true));
    let mut room = [NamespaceScope::EMPTY; 4];
    drhd.extend([SCOPE_IOAPIC, 7, 0, 0, 4, 0, 0x1F]);
    assert_eq!(walk_numbered_scopes(&drhd, 16, SCOPE_IOAPIC, &mut room), (2, true));
}
