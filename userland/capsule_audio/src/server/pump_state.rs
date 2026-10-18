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

//! What the pump keeps between steps: the mixer, a period waiting on a full
//! driver, how many went out, whether the driver's stream runs, and the
//! master volume.

use super::pump::PERIOD_SAMPLES;
use crate::mixer::Mixer;
use crate::volume::Volume;
use nonos_audio_proto::MasterVolume;

pub struct PumpState {
    pub(super) mixer: Mixer,
    pub(super) pending: Option<[i16; PERIOD_SAMPLES]>,
    pub(super) sent: usize,
    pub(super) running: bool,
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
    /// The output moved to another driver: the next step starts it.
    pub fn moved(&mut self) {
        self.running = false;
    }
    #[cfg(feature = "nonos-audio-smoketest")]
    pub fn sent(&self) -> usize {
        self.sent
    }
}
