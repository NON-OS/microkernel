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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::entry::file_name;
use super::kind::{kind_of, strip_ext, MediaKind};

/// Left this close to its end, a video counts as watched through.
pub const END_SLACK_MS: i64 = 2000;

pub struct MediaItem {
    pub path: String,
    pub name: String,
    pub kind: MediaKind,
    pub size: u64,
    pub duration_ms: i64,
    pub width: u32,
    pub height: u32,
    /// Where play was left in this window (`app/resume.rs`); 0 is unwatched.
    pub resume_ms: i64,
    pub thumb: Option<Vec<u32>>,
}

impl MediaItem {
    pub fn from_path(path: &str) -> Option<MediaItem> {
        let name = file_name(path);
        let kind = kind_of(name)?;
        Some(MediaItem {
            path: path.to_string(),
            name: name.to_string(),
            kind,
            size: 0,
            duration_ms: 0,
            width: 0,
            height: 0,
            resume_ms: 0,
            thumb: None,
        })
    }

    pub fn title(&self) -> &str {
        strip_ext(&self.name)
    }

    pub fn decodable(&self) -> bool {
        self.kind.decodable()
    }

    /// Where to open the video again: where it was left, or the start when
    /// that was within `END_SLACK_MS` of its end, as after watching it through.
    pub fn resume_point(&self) -> i64 {
        if self.resume_ms > 0 && self.resume_ms.saturating_add(END_SLACK_MS) < self.duration_ms {
            self.resume_ms
        } else {
            0
        }
    }

    pub fn permille(&self) -> u32 {
        if self.duration_ms <= 0 || self.resume_ms <= 0 {
            return 0;
        }
        let done = self.resume_ms.min(self.duration_ms) as u64 * 1000;
        (done / self.duration_ms as u64).min(1000) as u32
    }
}
