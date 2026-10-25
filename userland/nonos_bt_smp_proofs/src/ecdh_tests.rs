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

//! P-256 for LE Secure Connections: the published debug pair (Vol 3,
//! Part H, 2.3.5.6.1), two pairs agreeing on one DHKey, and the peer keys
//! and secrets that are refused.

use crate::hex::arr;
use nonos_bt_smp::ecdh::{KeyPair, PeerKey, DEBUG_PUBLIC_X, DEBUG_PUBLIC_Y, DEBUG_SECRET};
use nonos_bt_smp::SmpError;

/// The order n of P-256, which no secret may reach.
const N: &str = "ffffffff 00000000 ffffffff ffffffff bce6faad a7179e84 f3b9cac2 fc632551";

#[test]
fn the_debug_secret_gives_the_debug_public_key() {
    let pair = KeyPair::from_secret(DEBUG_SECRET).unwrap();
    assert_eq!(pair.public(), (&DEBUG_PUBLIC_X, &DEBUG_PUBLIC_Y));
}

#[test]
fn two_pairs_agree_on_the_dhkey() {
    let a = KeyPair::from_secret([0x11; 32]).unwrap();
    let b = KeyPair::from_secret([0x22; 32]).unwrap();
    let (ax, ay) = a.public();
    let (bx, by) = b.public();
    let (pa, pb) = (PeerKey::new(ax, ay).unwrap(), PeerKey::new(bx, by).unwrap());
    let shared = a.dh_key(&pb).unwrap();
    assert_eq!(shared, b.dh_key(&pa).unwrap());
    assert_ne!(shared, [0; 32]);
    assert_eq!(pa.x(), ax);
}

#[test]
fn off_curve_infinity_and_debug_peer_keys_are_refused() {
    assert_eq!(PeerKey::new(&DEBUG_PUBLIC_X, &DEBUG_PUBLIC_Y).err(), Some(SmpError::DebugKey));
    let mut y = DEBUG_PUBLIC_Y;
    y[31] ^= 1;
    assert_eq!(PeerKey::new(&DEBUG_PUBLIC_X, &y).err(), Some(SmpError::BadKey));
    assert_eq!(PeerKey::new(&[0; 32], &[0; 32]).err(), Some(SmpError::BadKey));
}

#[test]
fn zero_and_out_of_range_secrets_are_refused() {
    assert_eq!(KeyPair::from_secret([0; 32]).err(), Some(SmpError::BadSecret));
    assert_eq!(KeyPair::from_secret(arr(N)).err(), Some(SmpError::BadSecret));
    assert_eq!(KeyPair::from_secret([0xff; 32]).err(), Some(SmpError::BadSecret));
    let mut below = arr::<32>(N);
    below[31] -= 1;
    assert!(KeyPair::from_secret(below).is_ok());
}
