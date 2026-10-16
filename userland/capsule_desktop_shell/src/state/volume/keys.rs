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

//! The volume keys, and the volume and request a press asks for.

use nonos_app_skeleton::{KEY_MUTE, KEY_VOLUME_DOWN, KEY_VOLUME_UP};
use nonos_audio_proto::{volume_request, MasterVolume, VOLUME_MAX};

/// One press moves the level this far, out of `VOLUME_MAX`: twenty steps from
/// silence to full, fine enough to land on a comfortable level and few enough
/// to cross in a couple of seconds of a held key.
pub const VOLUME_STEP: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VolumeKey {
    Down,
    Up,
    Mute,
}

/// The volume key `code` names, or None for any other key.
pub fn volume_key(code: u32) -> Option<VolumeKey> {
    match code {
        KEY_VOLUME_DOWN => Some(VolumeKey::Down),
        KEY_VOLUME_UP => Some(VolumeKey::Up),
        KEY_MUTE => Some(VolumeKey::Mute),
        _ => None,
    }
}

/// The volume a press of `key` asks for, from `now`. A step clamps at silence
/// and at full rather than wrapping. A step either way also unmutes, as on
/// every desktop a person has used: pressing Volume Up and hearing nothing
/// reads as a broken key.
pub fn after_key(now: MasterVolume, key: VolumeKey) -> MasterVolume {
    match key {
        VolumeKey::Up => MasterVolume {
            level: now.level.saturating_add(VOLUME_STEP).min(VOLUME_MAX),
            muted: false,
        },
        VolumeKey::Down => MasterVolume {
            level: now.level.min(VOLUME_MAX).saturating_sub(VOLUME_STEP),
            muted: false,
        },
        VolumeKey::Mute => MasterVolume { level: now.level.min(VOLUME_MAX), muted: !now.muted },
    }
}

/// The volume a press of `key` asks for from `now`, and its request to the
/// audio service laid into `out`: the bytes written, zero when `out` is too
/// short to hold one.
pub fn press(
    now: MasterVolume,
    key: VolumeKey,
    request_id: u32,
    out: &mut [u8],
) -> (MasterVolume, usize) {
    let want = after_key(now, key);
    (want, volume_request(out, request_id, want))
}
