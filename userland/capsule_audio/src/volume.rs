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

//! The master volume as the mixer applies it: one gain over every sample it
//! writes out, streams and tones alike.
//!
//! The ear hears loudness roughly by the logarithm of amplitude, so a gain
//! straight from the level would put nearly all the change in the top few
//! steps: 50 would sound barely quieter than 100. The gain is the square of
//! the level instead, -12 dB at 50, -28 dB at 20 and -52 dB at 5, so each
//! step down sounds about as much quieter as the one before it, and 0 is
//! silence.
//!
//! Fixed point, 16 fractional bits, so the per-sample work is one multiply
//! and one shift on a gain worked out once, when the level changes.

use nonos_audio_proto::{MasterVolume, VOLUME_MAX};

/// The gain that plays a sample as it was mixed.
pub const UNITY: i32 = 1 << 16;

/// The gain for `volume`: `UNITY` at the top level, 0 at level 0 or muted,
/// rounded to the nearest step between. Never decreases as the level rises.
pub fn gain(volume: MasterVolume) -> i32 {
    if volume.muted {
        return 0;
    }
    let level = i32::from(volume.level.min(VOLUME_MAX));
    let full = i32::from(VOLUME_MAX) * i32::from(VOLUME_MAX);
    // 100 * 100 * 2^16 is 655 360 000, well inside an i32.
    (level * level * UNITY + full / 2) / full
}

/// `sample` at `gain`, rounded to the nearest step. A gain from `gain` is at
/// most `UNITY`, so the product stays inside an i32 (32767 * 2^16 plus the
/// rounding half is below 2^31) and the result is never louder than the
/// sample, so it cannot leave the i16 range.
pub fn scale(sample: i16, gain: i32) -> i16 {
    ((i32::from(sample) * gain + (1 << 15)) >> 16) as i16
}

/// The master volume in force, with its gain worked out once.
#[derive(Clone, Copy)]
pub struct Volume {
    setting: MasterVolume,
    gain: i32,
}

impl Volume {
    pub const fn new() -> Self {
        Self { setting: MasterVolume::FULL, gain: UNITY }
    }

    pub fn setting(&self) -> MasterVolume {
        self.setting
    }

    pub fn gain(&self) -> i32 {
        self.gain
    }

    pub fn set(&mut self, setting: MasterVolume) {
        self.setting = setting;
        self.gain = gain(setting);
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::new()
    }
}
