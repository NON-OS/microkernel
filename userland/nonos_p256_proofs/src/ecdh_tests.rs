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

//! ECDH: the published debug key pair (Bluetooth Core Specification 5.4,
//! Vol 3, Part H, 2.3.5.6.1), and two secrets agreeing on one shared x,
//! the value the separate big-integer implementation gave.

use crate::hex::arr;
use nonos_p256::*;

const DEBUG_SECRET: &str = "3f49f6d4a3c55f3874c9b3e3d2103f504aff607beb40b7995899b8a6cd3c1abd";
const DEBUG_X: &str = "20b003d2f297be2c5e2c83a7e9f9a5b9eff49111acf4fddbcc0301480e359de6";
const DEBUG_Y: &str = "dc809c49652aeb6d63329abf5a52155c766345c28fed3024741c8ed01589d28b";
const OTHER: &str = "519b423d715f8b581f4fa8ee59f4771a5b44c8130b4e3eacca54a56dda72b464";
const SHARED_X: &str = "f69431da15a8c13110b73e81657b7ed74b90b5bf952c16a9e978913788774eff";

fn point(xy: &[u8; 64]) -> ProjectivePoint {
    point_from_xy(xy[..32].try_into().unwrap(), xy[32..].try_into().unwrap()).unwrap()
}

#[test]
fn the_debug_secret_gives_the_debug_public_key() {
    let s = Secret::from_bytes(&arr::<32>(DEBUG_SECRET)).unwrap();
    let public = s.public_xy().unwrap();
    assert_eq!(public[..32], arr::<32>(DEBUG_X));
    assert_eq!(public[32..], arr::<32>(DEBUG_Y));
}

#[test]
fn both_sides_reach_the_same_x() {
    let a = Secret::from_bytes(&arr::<32>(DEBUG_SECRET)).unwrap();
    let b = Secret::from_bytes(&arr::<32>(OTHER)).unwrap();
    let (pa, pb) = (point(&a.public_xy().unwrap()), point(&b.public_xy().unwrap()));
    assert_eq!(a.shared_x(&pb), Some(arr::<32>(SHARED_X)));
    assert_eq!(b.shared_x(&pa), Some(arr::<32>(SHARED_X)));
}
