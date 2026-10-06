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

//! Where each video was left, for this window. The library cards and the
//! details page drew a watched bar and a resume time from a field nothing
//! wrote, so they always read nothing watched. The player now notes the
//! position when it is paused, left, ended or swapped for another video, and
//! opens a video there again. It is kept in memory only, like the rest of
//! the window's state.

use crate::catalog::media::MediaItem;
use crate::player::{duration_ms, Clock};

use super::state::VideoApp;

impl VideoApp {
    /// The video the player has open: a library entry, or the one handed
    /// over from outside the folders the library reads.
    pub(crate) fn playing_item(&self) -> Option<&MediaItem> {
        self.browse
            .item_by_path(&self.path)
            .or(self.outside.as_ref().filter(|m| m.path == self.path))
    }

    /// Record where play stands on the open video; at the end, its length.
    pub(crate) fn note_position(&mut self) {
        let Some(file) = self.file.as_ref() else { return };
        let total = file.index.len() as u32;
        let upf = file.header.micro_sec_per_frame;
        let ms = if self.next >= total {
            duration_ms(total, upf)
        } else {
            Clock::new(0, 0, upf).pts_ms(self.next)
        };
        let VideoApp { browse, outside, path, .. } = self;
        let item = match browse.item_by_path_mut(path) {
            Some(item) => Some(item),
            None => outside.as_mut().filter(|m| m.path == *path),
        };
        if let Some(item) = item {
            item.resume_ms = ms;
        }
    }

    /// The frame to open the video at: where it was left, unless that was
    /// at or near its end.
    pub(super) fn start_frame(&self, total: u32, usec_per_frame: u32) -> u32 {
        let ms = self.playing_item().map_or(0, |m| m.resume_point());
        if ms <= 0 || usec_per_frame == 0 {
            return 0;
        }
        let frame = (ms as u64).saturating_mul(1000) / usec_per_frame as u64;
        frame.min(total.saturating_sub(1) as u64) as u32
    }
}
