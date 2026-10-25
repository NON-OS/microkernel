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

//! The volume keys' steps, clamps and mute.

use super::rig::v;
use crate::shell_system_keys::volume::keys::{after_key, VOLUME_STEP};
use crate::shell_system_keys::volume::VolumeKey;
use nonos_audio_proto::VOLUME_MAX;

#[test]
fn up_and_down_step_by_five_and_clamp() {
    assert_eq!(VOLUME_STEP, 5);
    assert_eq!(after_key(v(40, false), VolumeKey::Up), v(45, false));
    assert_eq!(after_key(v(40, false), VolumeKey::Down), v(35, false));
    assert_eq!(after_key(v(100, false), VolumeKey::Up), v(100, false), "clamped at full");
    assert_eq!(after_key(v(98, false), VolumeKey::Up), v(100, false));
    assert_eq!(after_key(v(0, false), VolumeKey::Down), v(0, false), "clamped at silence");
    assert_eq!(after_key(v(3, false), VolumeKey::Down), v(0, false));
}

#[test]
fn a_held_key_walks_the_whole_range_and_stops_at_each_end() {
    // A held key repeats as more presses, a step each.
    let mut now = v(0, false);
    for _ in 0..30 {
        now = after_key(now, VolumeKey::Up);
        assert!(now.level <= VOLUME_MAX);
    }
    assert_eq!(now, v(100, false));
    for _ in 0..30 {
        now = after_key(now, VolumeKey::Down);
    }
    assert_eq!(now, v(0, false));
    let presses = (0..).scan(v(0, false), |s, _| {
        *s = after_key(*s, VolumeKey::Up);
        Some(s.level)
    });
    assert_eq!(presses.take_while(|&l| l < 100).count() + 1, 20, "twenty steps to full");
}

#[test]
fn mute_flips_and_keeps_the_level_and_a_step_unmutes() {
    let muted = after_key(v(60, false), VolumeKey::Mute);
    assert_eq!(muted, v(60, true));
    assert_eq!(after_key(muted, VolumeKey::Mute), v(60, false));
    assert_eq!(after_key(muted, VolumeKey::Up), v(65, false));
    assert_eq!(after_key(muted, VolumeKey::Down), v(55, false));
}
