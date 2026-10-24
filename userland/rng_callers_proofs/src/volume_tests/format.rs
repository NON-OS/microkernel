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

//! Formatting a volume under a fresh key (format_volume.rs).

use super::super::error::VolumeError;
use super::super::format_volume::format_volume;
use super::{open_key, turn};
use crate::stand_in::{keyring, rng, volume};

#[test]
fn no_volume_is_formatted_with_no_randomness() {
    let _turn = turn();
    rng::refuse();
    assert_eq!(format_volume([3; 16]), Err(VolumeError::NoRandomness));
    assert!(keyring::stored().is_empty());
    assert!(volume::formats().is_empty());
    assert_eq!(open_key(), None);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn the_volume_key_is_drawn_stored_and_formatted_under() {
    let _turn = turn();
    rng::fail_from(None);
    assert_eq!(format_volume([3; 16]), Ok(1));
    let key = rng::given()[0].clone();
    assert_eq!(keyring::stored(), core::slice::from_ref(&key));
    let formatted: Vec<_> = volume::formats().iter().map(|f| f.0.to_vec()).collect();
    assert_eq!(formatted, core::slice::from_ref(&key));
    assert_eq!(open_key().map(|k| k.to_vec()), Some(key.clone()));
    format_volume([3; 16]).expect("formatted");
    assert_ne!(keyring::stored()[1], key);
    assert_eq!(rng::weak(), 0);
}
