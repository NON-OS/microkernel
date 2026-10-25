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

//! K3: the IO-APIC scopes a DRHD names, and each one's requester id.

use super::dmar_ioapic::{ioapic_scopes, IoapicScope, SCOPE_IOAPIC};
use super::dmar_ioapic_source::source_id;

/// A DRHD's 16-byte header (the walk reads none of it), then `scopes`.
fn drhd(scopes: &[&[u8]]) -> Vec<u8> {
    let mut b = vec![0u8; 16];
    scopes.iter().for_each(|s| b.extend_from_slice(s));
    b
}

fn parse(bytes: &[u8]) -> Vec<IoapicScope> {
    let mut out = [None; 2];
    ioapic_scopes(bytes, 16, &mut out);
    out.iter().flatten().copied().collect()
}

#[test]
fn only_ioapic_scopes_are_kept_with_their_id_and_path() {
    let endpoint: &[u8] = &[1, 8, 0, 0, 0, 0, 0x14, 0];
    let ioapic: &[u8] = &[SCOPE_IOAPIC, 8, 0, 0, 0x08, 0xF0, 0x1F, 0];
    let two_hops: &[u8] = &[SCOPE_IOAPIC, 10, 0, 0, 0x09, 0, 0x1C, 4, 0, 0];
    let got = parse(&drhd(&[endpoint, ioapic, two_hops]));
    assert_eq!(got.len(), 2);
    assert_eq!((got[0].id, got[0].start_bus, got[0].path[0]), (8, 0xF0, (0x1F, 0)));
    assert_eq!((got[1].id, got[1].path_len, got[1].path[1]), (9, 2, (0, 0)));
}

#[test]
fn hostile_lengths_stop_the_walk_and_surplus_is_dropped() {
    let pathless: &[u8] = &[SCOPE_IOAPIC, 6, 0, 0, 1, 0];
    let ok: &[u8] = &[SCOPE_IOAPIC, 8, 0, 0, 2, 0, 0x1F, 0];
    assert_eq!(parse(&drhd(&[pathless, ok])).len(), 1, "a pathless scope is skipped");
    let odd: &[u8] = &[SCOPE_IOAPIC, 9, 0, 0, 3, 0, 0x1F, 0, 0];
    assert!(parse(&drhd(&[odd, ok])).is_empty(), "an odd length ends the list");
    let past: &[u8] = &[SCOPE_IOAPIC, 0xFF, 0, 0, 5, 0, 0x1F, 0];
    assert!(parse(&drhd(&[past])).is_empty(), "a scope past the structure");
    assert_eq!(parse(&drhd(&[ok, ok, ok])).len(), 2, "no more than the table holds");
    let five_hops: &[u8] = &[SCOPE_IOAPIC, 16, 0, 0, 4, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    assert!(parse(&drhd(&[five_hops])).is_empty(), "a path past MAX_PATH");
}

fn at(start_bus: u8, hops: &[(u8, u8)]) -> IoapicScope {
    let mut path = [(0, 0); 4];
    path[..hops.len()].copy_from_slice(hops);
    IoapicScope { id: 8, start_bus, path, path_len: hops.len() as u8 }
}

#[test]
fn the_requester_id_follows_each_bridge() {
    let no = |_, _, _| None;
    assert_eq!(source_id(&at(0xF0, &[(0x1F, 0)]), no), Some(0xF0F8), "Linux: 0xf0:1f.0");
    let bridge = |b, d, f| ((b, d, f) == (0xF0, 0x1C, 4)).then_some(0x05);
    assert_eq!(source_id(&at(0xF0, &[(0x1C, 4), (0, 2)]), bridge), Some(0x0502));
    assert_eq!(source_id(&at(0xF0, &[(0x1C, 4), (0, 2)]), no), None, "unreadable bridge");
    assert_eq!(source_id(&at(0, &[(32, 0)]), no), None, "device past 31");
    assert_eq!(source_id(&at(0, &[(0, 8)]), no), None, "function past 7");
}
