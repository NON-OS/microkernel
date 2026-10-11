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

//! One period mixed from every active stream, and its bytes on the wire.

use super::pump::{KEEP_AHEAD, PERIOD_SAMPLES};
use super::streams::StreamTable;
use crate::mark::mark;
use crate::mixer::Mixer;

pub(super) fn mix(mixer: &mut Mixer, table: &mut StreamTable, gain: i32) -> [i16; PERIOD_SAMPLES] {
    mixer.clear();
    for slot in table.iter_active() {
        let mut tmp = [0i16; PERIOD_SAMPLES];
        slot.feed.pop_period(&mut tmp);
        mixer.add(&tmp);
    }
    let mut bytes = [0u8; PERIOD_SAMPLES * 2];
    mixer.write_bytes(&mut bytes, gain);
    let mut out = [0i16; PERIOD_SAMPLES];
    for i in 0..PERIOD_SAMPLES {
        out[i] = i16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
    }
    out
}

pub(super) fn to_bytes(samples: &[i16; PERIOD_SAMPLES]) -> [u8; PERIOD_SAMPLES * 2] {
    let mut out = [0u8; PERIOD_SAMPLES * 2];
    for i in 0..PERIOD_SAMPLES {
        out[i * 2..i * 2 + 2].copy_from_slice(&samples[i].to_le_bytes());
    }
    out
}

/// Marked once, when the first periods have gone out; after that the count
/// alone grows, so a long playback does not write a line every few periods.
pub(super) fn mark_progress(sent: usize) {
    if sent == KEEP_AHEAD {
        mark("[AUDIO] pump\n");
    }
}
