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

//! A live boot's volume in RAM (session.rs): its key and uuid are drawn
//! before anything is started, so with none, no RAM disk is started, no
//! window is set and nothing is formatted.

use super::super::error::VolumeError;
use super::super::session::open_session_volume;
use super::{open_key, turn};
use crate::fs::cryptoblock::window_sectors;
use crate::stand_in::{ram, rng, volume};

#[test]
fn no_session_volume_is_started_when_either_draw_refuses() {
    let _turn = turn();
    let window = window_sectors();
    for (from, only) in rng::cases(2) {
        rng::fail_at(Some(from), only);
        assert_eq!(open_session_volume(false), Err(VolumeError::NoRandomness));
        assert_eq!(ram::ram_started(), None);
        assert_eq!(window_sectors(), window);
        assert!(volume::formats().is_empty());
        assert_eq!(open_key(), None);
    }
    assert_eq!(rng::weak(), 0);
}

#[test]
fn the_session_volume_is_made_under_its_drawn_key_and_uuid() {
    let _turn = turn();
    rng::fail_from(None);
    assert_eq!(open_session_volume(false), Ok(()));
    let given = rng::given();
    let (key, uuid) = volume::formats()[0];
    assert_eq!((key.to_vec(), uuid.to_vec()), (given[0].clone(), given[1].clone()));
    assert_eq!(open_key(), Some(key));
    let sectors = ram::ram_started().expect("RAM disk started");
    assert_eq!(window_sectors(), Some(sectors));
    *super::VOLUME.write() = None;
    open_session_volume(false).expect("opened");
    assert_ne!(volume::formats()[1].0, key);
    assert_eq!(rng::weak(), 0);
}
