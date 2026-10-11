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

//! The codec's output levels. Open while a stream runs, master, headphone
//! and PCM out unmuted at full level (the audio server scales the samples
//! to the user's volume); muted whenever nothing plays, so a stopped engine
//! can never be heard, as the HD Audio driver closes its amplifiers.

use crate::constants::mixer::{FULL_LEVEL, HEADPHONE, MASTER, MUTE, PCM_OUT, PCM_UNITY};
use crate::regs::Mixer;

pub fn open(m: &Mixer, on: bool) {
    let mute = if on { 0 } else { MUTE };
    m.write(PCM_OUT, PCM_UNITY | mute);
    m.write(MASTER, FULL_LEVEL | mute);
    m.write(HEADPHONE, FULL_LEVEL | mute);
}
