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

use super::scope::{parse_drhd, scope_covers, translated_by_some, Cover, UnitScope, MAX_SCOPES};

/* A DRHD: type 0, length, flags, size, segment, register base, then scopes. */
fn drhd(flags: u8, segment: u16, scopes: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![0u8, 0, 0, 0, flags, 0];
    b.extend_from_slice(&segment.to_le_bytes());
    b.extend_from_slice(&0xFED9_0000u64.to_le_bytes());
    for s in scopes {
        b.extend_from_slice(s);
    }
    let len = b.len() as u16;
    b[2..4].copy_from_slice(&len.to_le_bytes());
    b
}

/* A device scope: type, length, reserved, enumeration id, start bus, path. */
fn scope(kind: u8, start_bus: u8, path: &[(u8, u8)]) -> Vec<u8> {
    let mut s = vec![kind, (6 + 2 * path.len()) as u8, 0, 0, 0, start_bus];
    for &(d, f) in path {
        s.push(d);
        s.push(f);
    }
    s
}

/* Bus 0 has a root port at 00:1c.0 whose secondary bus is 1 (where an M.2 Wi-Fi
card sits, 01:00.0), and one at 00:1d.0 bridging buses 2..=3. */
fn bridges(bus: u8, dev: u8, func: u8) -> Option<(u8, u8)> {
    match (bus, dev, func) {
        (0, 0x1c, 0) => Some((1, 1)),
        (0, 0x1d, 0) => Some((2, 3)),
        _ => None,
    }
}

fn parse(b: &[u8]) -> UnitScope {
    parse_drhd(b).expect("a DRHD parses")
}

/* Shaped like a Gemini Lake laptop: the graphics unit names 00:02.0, and the
last unit covers everything else. */
fn laptop() -> [UnitScope; 2] {
    [
        parse(&drhd(0, 0, &[scope(1, 0, &[(2, 0)])])),
        parse(&drhd(1, 0, &[scope(3, 0, &[(0x1f, 0)])])),
    ]
}

#[test]
fn flags_segment_and_scopes_are_read() {
    let u = parse(&drhd(1, 0, &[scope(1, 0, &[(2, 0)]), scope(2, 0, &[(0x1d, 0)])]));
    assert!(u.include_all);
    assert_eq!(u.segment, 0);
    assert_eq!(u.count, 2);
    assert_eq!(u.scopes[0].kind, 1);
    assert_eq!(&u.scopes[0].path[..1], &[(2, 0)]);
    assert!(!u.truncated);
}

#[test]
fn ioapic_and_hpet_scopes_name_no_pci_device() {
    let u = parse(&drhd(1, 0, &[scope(3, 0xf0, &[(0x1f, 0)]), scope(4, 0, &[(0x1f, 0)])]));
    assert_eq!(u.count, 0);
    assert!(!u.truncated);
}

#[test]
fn a_short_or_mistyped_structure_is_not_a_drhd() {
    assert!(parse_drhd(&[0u8; 8]).is_none());
    let mut b = drhd(0, 0, &[]);
    b[0] = 1;
    assert!(parse_drhd(&b).is_none());
}

#[test]
fn a_malformed_scope_marks_the_unit_truncated() {
    let mut b = drhd(0, 0, &[scope(1, 0, &[(2, 0)])]);
    b.extend_from_slice(&[1, 3, 0]);
    let len = b.len() as u16;
    b[2..4].copy_from_slice(&len.to_le_bytes());
    let u = parse(&b);
    assert_eq!(u.count, 1);
    assert!(u.truncated);
}

#[test]
fn more_scopes_than_kept_marks_the_unit_truncated() {
    let many: Vec<Vec<u8>> = (0..MAX_SCOPES as u8 + 1).map(|d| scope(1, 0, &[(d, 0)])).collect();
    let u = parse(&drhd(0, 0, &many));
    assert_eq!(u.count as usize, MAX_SCOPES);
    assert!(u.truncated);
}

#[test]
fn an_endpoint_scope_covers_exactly_its_device() {
    let [gfx, _] = laptop();
    assert_eq!(scope_covers(&gfx, 0, (0, 2, 0), &bridges), Cover::Yes);
    assert_eq!(scope_covers(&gfx, 0, (1, 0, 0), &bridges), Cover::No);
    assert_eq!(scope_covers(&gfx, 1, (0, 2, 0), &bridges), Cover::No);
}

#[test]
fn a_multi_hop_path_walks_through_the_bridge() {
    let u = parse(&drhd(0, 0, &[scope(1, 0, &[(0x1c, 0), (0, 0)])]));
    assert_eq!(scope_covers(&u, 0, (1, 0, 0), &bridges), Cover::Yes);
    assert_eq!(scope_covers(&u, 0, (0, 0, 0), &bridges), Cover::No);
}

#[test]
fn a_bridge_scope_covers_its_subtree() {
    let u = parse(&drhd(0, 0, &[scope(2, 0, &[(0x1d, 0)])]));
    assert_eq!(scope_covers(&u, 0, (0, 0x1d, 0), &bridges), Cover::Yes);
    assert_eq!(scope_covers(&u, 0, (2, 0, 0), &bridges), Cover::Yes);
    assert_eq!(scope_covers(&u, 0, (3, 5, 1), &bridges), Cover::Yes);
    assert_eq!(scope_covers(&u, 0, (1, 0, 0), &bridges), Cover::No);
}

#[test]
fn a_bridge_that_does_not_answer_leaves_the_answer_unknown() {
    let u = parse(&drhd(0, 0, &[scope(2, 0, &[(0x1e, 0)])]));
    assert_eq!(scope_covers(&u, 0, (4, 0, 0), &bridges), Cover::Unknown);
}

/* The RTL8821CE fault: the Wi-Fi card at 01:00.0 is behind the catch-all unit,
not the graphics unit that used to be the only one programmed. */
#[test]
fn the_wifi_card_is_the_catch_all_units_not_the_graphics_units() {
    let [gfx, rest] = laptop();
    assert_eq!(scope_covers(&gfx, 0, (1, 0, 0), &bridges), Cover::No);
    assert!(!translated_by_some(&[gfx], 0, (1, 0, 0), &bridges));
    assert!(translated_by_some(&[gfx, rest], 0, (1, 0, 0), &bridges));
    assert!(translated_by_some(&[gfx, rest], 0, (0, 2, 0), &bridges));
}

#[test]
fn with_no_catch_all_unit_an_unnamed_device_is_not_translated() {
    let gfx = parse(&drhd(0, 0, &[scope(1, 0, &[(2, 0)])]));
    let storage = parse(&drhd(0, 0, &[scope(2, 0, &[(0x1d, 0)])]));
    let units = [gfx, storage];
    assert!(translated_by_some(&units, 0, (2, 0, 0), &bridges));
    assert!(!translated_by_some(&units, 0, (1, 0, 0), &bridges));
}

#[test]
fn an_unresolvable_scope_is_never_taken_for_coverage() {
    let u = parse(&drhd(0, 0, &[scope(1, 0, &[(0x1e, 0), (0, 0)])]));
    assert!(!translated_by_some(&[u], 0, (5, 0, 0), &bridges));
    let mut t = UnitScope::EMPTY;
    t.truncated = true;
    assert!(!translated_by_some(&[t], 0, (0, 2, 0), &bridges));
}

#[test]
fn no_units_translate_nothing() {
    assert!(!translated_by_some(&[], 0, (0, 2, 0), &bridges));
}

#[test]
fn a_catch_all_unit_covers_only_its_own_segment() {
    let u = parse(&drhd(1, 1, &[]));
    assert!(!translated_by_some(&[u], 0, (1, 0, 0), &bridges));
    assert!(translated_by_some(&[u], 1, (1, 0, 0), &bridges));
}
