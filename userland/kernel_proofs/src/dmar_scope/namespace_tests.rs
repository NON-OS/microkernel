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

//! The namespace device scopes under a DRHD, and the requester id each one
//! gives a device the kernel never enumerates.

use super::dmar_namespace_scope::{NamespaceScope, SCOPE_NAMESPACE};
use super::dmar_namespace_walk::walk_namespace_scopes;

fn scope(kind: u8, number: u8, bus: u8, hops: &[(u8, u8)]) -> Vec<u8> {
    let mut b = vec![kind, (6 + hops.len() * 2) as u8, 0, 0, number, bus];
    hops.iter().for_each(|(d, f)| b.extend_from_slice(&[*d, *f]));
    b
}

#[test]
fn the_walk_keeps_namespace_scopes_and_their_requester_ids() {
    let mut drhd = vec![0u8; 16];
    drhd.extend(scope(1, 0, 0, &[(0x1F, 3)]));
    drhd.extend(scope(SCOPE_NAMESPACE, 1, 0, &[(0x10, 0)]));
    drhd.extend(scope(SCOPE_NAMESPACE, 2, 0, &[(0x1C, 0), (0, 0)]));
    let mut out = [NamespaceScope::EMPTY; 4];
    assert_eq!(walk_namespace_scopes(&drhd, 16, &mut out), (2, false));
    assert_eq!((out[0].number, out[0].requester()), (1, Some((0, 0x10, 0))));
    // Through a bridge: the bus below it is enumeration's to say, not guessed.
    assert_eq!((out[1].number, out[1].requester()), (2, None));
    let bad = NamespaceScope { path: [(32, 0); 4], path_len: 1, ..NamespaceScope::EMPTY };
    assert_eq!(bad.requester(), None);
}

#[test]
fn a_malformed_or_crowded_scope_list_says_so() {
    let mut drhd = vec![0u8; 16];
    drhd.extend(scope(SCOPE_NAMESPACE, 1, 0, &[(0x10, 0)]));
    drhd.extend(scope(SCOPE_NAMESPACE, 2, 0, &[(0x10, 1)]));
    let mut one = [NamespaceScope::EMPTY; 1];
    assert_eq!(walk_namespace_scopes(&drhd, 16, &mut one), (1, true));
    drhd.extend([SCOPE_NAMESPACE, 7, 0, 0, 3, 0, 0x10]);
    let mut room = [NamespaceScope::EMPTY; 4];
    assert_eq!(walk_namespace_scopes(&drhd, 16, &mut room), (2, true));
    let empty_path = [vec![0u8; 16], scope(SCOPE_NAMESPACE, 1, 0, &[])].concat();
    assert_eq!(walk_namespace_scopes(&empty_path, 16, &mut room), (0, true));
}
