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

use super::proto::E_AGAIN;
use super::streams::StreamTable;
use crate::mark::mark;
use crate::mixer::Mixer;
use crate::sink::Sink;
use crate::volume::Volume;
use nonos_audio_proto::MasterVolume;

pub const PERIOD_FRAMES: usize = 1024;
pub const PERIOD_SAMPLES: usize = PERIOD_FRAMES * 2;
pub const KEEP_AHEAD: usize = 3;
pub struct PumpState {
    mixer: Mixer,
    pending: Option<[i16; PERIOD_SAMPLES]>,
    sent: usize,
    running: bool,
    // The master volume, held with the output it shapes. Full and not muted
    // until a client sets it, so nothing plays quieter than it did before
    // there was a master volume.
    volume: Volume,
}
impl PumpState {
    pub fn new() -> Self {
        Self { mixer: Mixer::new(), pending: None, sent: 0, running: false, volume: Volume::new() }
    }
    pub fn volume(&self) -> MasterVolume {
        self.volume.setting()
    }
    /// The gain every sample written out is played at (crate::volume).
    pub fn gain(&self) -> i32 {
        self.volume.gain()
    }
    pub fn set_volume(&mut self, setting: MasterVolume) {
        self.volume.set(setting);
    }
    #[cfg(feature = "nonos-audio-smoketest")]
    pub fn sent(&self) -> usize {
        self.sent
    }
}

fn mix(mixer: &mut Mixer, table: &mut StreamTable, gain: i32) -> [i16; PERIOD_SAMPLES] {
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

fn to_bytes(samples: &[i16; PERIOD_SAMPLES]) -> [u8; PERIOD_SAMPLES * 2] {
    let mut out = [0u8; PERIOD_SAMPLES * 2];
    for i in 0..PERIOD_SAMPLES {
        out[i * 2..i * 2 + 2].copy_from_slice(&samples[i].to_le_bytes());
    }
    out
}

/// Marked once, when the first periods have gone out; after that the count
/// alone grows, so a long playback does not write a line every few periods.
fn mark_progress(sent: usize) {
    if sent == KEEP_AHEAD {
        mark("[AUDIO] pump\n");
    }
}

fn gate(state: &mut PumpState, table: &StreamTable, sink: &Sink) -> bool {
    if !table.any_active() {
        if state.running {
            state.pending = None;
            state.running = false;
            sink.stream_stop(state.sent as u32);
            mark("[AUDIO] dac-stop\n");
        }
        return false;
    }
    if !state.running {
        state.running = true;
        sink.stream_start(state.sent as u32);
        mark("[AUDIO] dac-start\n");
    }
    true
}

/// Keep the driver fed, every period mixed at the master volume. A period
/// already mixed and waiting on a full driver keeps the gain it was mixed at;
/// the next one takes the new one, a period (about 21 ms) later.
pub fn step(state: &mut PumpState, table: &mut StreamTable, sink: &Sink) {
    let gain = state.gain();
    if !gate(state, table, sink) {
        return;
    }
    if let Some(pending) = state.pending {
        let status = sink.write_pcm_status(&to_bytes(&pending), state.sent as u32);
        if status == E_AGAIN {
            return;
        }
        state.pending = None;
        state.sent += 1;
        mark_progress(state.sent);
    }
    for _ in 0..KEEP_AHEAD {
        let samples = mix(&mut state.mixer, table, gain);
        let status = sink.write_pcm_status(&to_bytes(&samples), state.sent as u32);
        if status == E_AGAIN {
            state.pending = Some(samples);
            break;
        }
        if status < 0 {
            break;
        }
        state.sent += 1;
        mark_progress(state.sent);
    }
}
