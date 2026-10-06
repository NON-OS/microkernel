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

//! The player's volume controls as the system's master volume: the slider's
//! position as a level, the keys' steps, and the level as the slider shows it.

use nonos_audio_proto::{MasterVolume, VOLUME_MAX};

/// One press of a volume key in the player, as the keyboard's keys step.
pub const STEP: u8 = 5;

/// A slider at `permille` (0 to 1000) as a level, not muted: moving the
/// slider is asking to hear it.
pub fn at_permille(permille: u32) -> MasterVolume {
    let level = (permille.min(1000) * u32::from(VOLUME_MAX) + 500) / 1000;
    MasterVolume { level: level as u8, muted: false }
}

/// `now` stepped up or down by `STEP`, kept in 0 to 100; a step unmutes.
pub fn stepped(now: MasterVolume, up: bool) -> MasterVolume {
    let level = if up {
        now.level.saturating_add(STEP).min(VOLUME_MAX)
    } else {
        now.level.saturating_sub(STEP)
    };
    MasterVolume { level, muted: false }
}

/// `now` with its mute switched.
pub fn toggled(now: MasterVolume) -> MasterVolume {
    MasterVolume { level: now.level, muted: !now.muted }
}

/// The level as the slider draws it, in the Q15 the bar reads.
pub fn q15(volume: MasterVolume) -> i32 {
    if volume.muted {
        return 0;
    }
    (i32::from(volume.level) << 15) / i32::from(VOLUME_MAX)
}
