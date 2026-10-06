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

//! The track the shell asked Music to open (the file manager's Enter on an
//! .mp3 or .wav). The shell keeps the path until the app it launched or
//! focused asks, so Music asks at start and then about once a second, which
//! also catches a file sent to a window already open. Once a second, not
//! every tick: a playing or loading track ticks every 10 ms.
//!
//! The file goes through the same stepped load as a track picked in the
//! library (`loading.rs`) and plays once it is ready. One in /audio selects
//! that entry; one from anywhere else plays as now playing without joining
//! the library, whose indices the queue holds.

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};
use nonos_libc::mk_uptime_ms;

use super::PlayerApp;
use crate::library::{handed, Handed, Track};
use crate::track::blank;
use crate::trouble::track_trouble;
use crate::ui::View;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and
// video_player's poll_open_arg: keep them identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;
const EVERY_MS: i64 = 1000;

/// What the decoder says of a file that is not MP3 or WAV, so the bar reads
/// the same as for such a file picked anywhere else.
const NOT_AUDIO: &str = "unknown audio format";

impl PlayerApp {
    /// Play the file the shell holds for this app, if any. True when the
    /// window changed.
    pub(super) fn poll_open_arg(&mut self) -> bool {
        let now = mk_uptime_ms();
        if now < self.arg_due_ms {
            return false;
        }
        self.arg_due_ms = now + EVERY_MS;
        let Some(shell) = lookup_service(b"desktop_shell") else { return false };
        let mut rx = [0u8; 512];
        let Ok(total) = call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx) else {
            return false;
        };
        let body = rx.get(HDR_LEN..total).unwrap_or(&[]);
        match handed(body, &self.library.tracks) {
            Handed::None => return false,
            Handed::InLibrary(i) => self.select(i),
            Handed::Outside(path) => {
                self.outside = Some(Track::from_path(path));
                self.load_outside();
            }
            Handed::Refuse(path) => self.refuse(Track::from_path(path)),
        }
        self.ui.go(View::NowPlaying);
        true
    }

    /// A handed-over file Music cannot decode: named in the bar with the
    /// reason, as a track that failed to load is, and nothing else played
    /// under its name.
    fn refuse(&mut self, track: Track) {
        self.transport.stop();
        self.loading = None;
        self.waveform = blank();
        self.meta.title = track.title.clone();
        self.meta.artist.clear();
        self.meta.format.clear();
        self.trouble = Some(track_trouble(NOT_AUDIO));
        self.outside = Some(track);
    }
}
