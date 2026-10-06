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

//! Music's volume, which is the system's, and its keys.

use nonos_app_skeleton::{KEY_DOWN, KEY_UP};
use nonos_audio_proto::MasterVolume;

use crate::audio_shortcut::{shortcut, Shortcut};
use crate::audio_volume::{at_permille, q15, stepped, toggled, STEP};

#[test]
fn the_slider_sets_a_level_and_unmutes() {
    assert_eq!(at_permille(0), MasterVolume { level: 0, muted: false });
    assert_eq!(at_permille(1000).level, 100);
    assert_eq!(at_permille(4_000).level, 100, "past the end is the end");
    assert_eq!(at_permille(455).level, 46, "rounded");
}

#[test]
fn steps_clamp_and_unmute() {
    let muted = MasterVolume { level: 98, muted: true };
    assert_eq!(stepped(muted, true), MasterVolume { level: 100, muted: false });
    assert_eq!(stepped(MasterVolume { level: 3, muted: false }, false).level, 0);
    assert_eq!(stepped(MasterVolume { level: 50, muted: false }, false).level, 50 - STEP);
    assert_eq!(toggled(muted), MasterVolume { level: 98, muted: false });
}

#[test]
fn the_slider_draws_the_level_and_shows_mute_as_silence() {
    assert_eq!(q15(MasterVolume::FULL), 1 << 15);
    assert_eq!(q15(MasterVolume { level: 50, muted: false }), 1 << 14);
    assert_eq!(q15(MasterVolume { level: 80, muted: true }), 0);
}

#[test]
fn keys_map_in_either_case() {
    assert_eq!(shortcut(KEY_UP), Some(Shortcut::VolumeUp));
    assert_eq!(shortcut(KEY_DOWN), Some(Shortcut::VolumeDown));
    assert_eq!(shortcut(b'm' as u32), Some(Shortcut::Mute));
    assert_eq!(shortcut(b'N' as u32), Some(Shortcut::Next));
    assert_eq!(shortcut(b'p' as u32), Some(Shortcut::Prev));
    assert_eq!(shortcut(b'S' as u32), Some(Shortcut::Shuffle));
    assert_eq!(shortcut(b'r' as u32), Some(Shortcut::Repeat));
    assert_eq!(shortcut(b'/' as u32), Some(Shortcut::Search));
    assert_eq!(shortcut(b'x' as u32), None);
    assert_eq!(shortcut(b' ' as u32), None, "space stays play and pause");
}
