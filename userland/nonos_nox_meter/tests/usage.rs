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

mod host;

use host::{keccak, Host};
use nonos_nox_meter::merkle::{hash_pair, root};
use nonos_nox_meter::usage::Charge;

#[test]
fn a_usage_leaf_is_the_oz_standard_leaf() {
    let c =
        Charge { channel_id: [1; 32], seq: 3, unit: [2; 32], qty: 5, price: 9, call_id: [4; 32] };
    let mut pre = [1u8; 32].to_vec();
    pre.extend_from_slice(&[[0u8; 24].as_slice(), &3u64.to_be_bytes()].concat());
    pre.extend_from_slice(&[2u8; 32]);
    pre.extend_from_slice(&[[0u8; 24].as_slice(), &5u64.to_be_bytes()].concat());
    pre.extend_from_slice(&[[0u8; 16].as_slice(), &9u128.to_be_bytes()].concat());
    pre.extend_from_slice(&[4u8; 32]);
    assert_eq!(c.leaf(&Host), keccak(&keccak(&pre)));
}

#[test]
fn the_root_is_sorted_pairs_and_order_free() {
    let (a, b, c) = ([3u8; 32], [1u8; 32], [2u8; 32]);
    assert_eq!(root(&Host, &[]), [0u8; 32]);
    assert_eq!(root(&Host, &[a]), a);
    let ab = keccak(&[b, a].concat());
    assert_eq!(hash_pair(&Host, &a, &b), ab);
    assert_eq!(root(&Host, &[a, b]), ab);
    // Sorted: b c a -> (b,c) then a carried up, then the pair of those.
    let bc = keccak(&[b, c].concat());
    let want = hash_pair(&Host, &bc, &a);
    assert_eq!(root(&Host, &[a, b, c]), want);
    assert_eq!(root(&Host, &[c, a, b]), want);
}
