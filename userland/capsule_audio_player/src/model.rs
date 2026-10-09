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

extern crate alloc;
use alloc::string::String;
use crate::transport::{State, Transport};
use crate::trouble::output_trouble;

pub struct TrackMeta { pub title: String, pub artist: String, pub format: String }

#[derive(Clone)]
pub struct PlayerView {
    pub title: String,
    pub artist: String,
    pub format: String,
    pub pos_ms: u32,
    pub dur_ms: u32,
    pub volume_q15: i32,
    pub state: State,
    pub muted: bool,
    pub shuffle: bool,
    pub repeat: bool,
    /// Why the player cannot play, drawn in red in place of the artist.
    pub notice: Option<&'static str>,
}

impl Transport {
    pub fn view(&self, meta: &TrackMeta) -> PlayerView {
        PlayerView {
            title: meta.title.clone(),
            artist: meta.artist.clone(),
            format: meta.format.clone(),
            pos_ms: self.frames_to_ms(self.pos_frames()),
            dur_ms: self.frames_to_ms(self.dur_frames()),
            volume_q15: self.volume_q15(),
            state: self.state(),
            muted: self.muted(),
            shuffle: false,
            repeat: false,
            notice: self.fault().map(output_trouble),
        }
    }
}
