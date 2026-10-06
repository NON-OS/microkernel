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

//! Downloads, seen from the window: an MP3 link pasted into Search joins the
//! Downloads page, which runs them one at a time on the worker (`fetch`); the
//! bar and the page say how each is going, and each finished file joins the
//! library. The first to finish while nothing plays starts playing.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use nonos_app_skeleton::clients::vfs;
use nonos_app_skeleton::clipboard_paste_line;
use nonos_libc::{mk_getpid, mk_uptime_ms};

use super::PlayerApp;
use crate::fetch::list::{Act, State};
use crate::fetch::{self, name::file_name, row_text};
use crate::library::{same_path, Track};
use crate::transport::State as Play;
use crate::ui::search_key::{paste_query, Pasted};
use crate::ui::View;

const PASTE_CUT: &str = "Only the start of what was copied fits the field; copy a shorter address";
const PASTE_NONE: &str = "The clipboard holds no text to paste; copy the address, then paste again";
const PASTE_UNREACHED: &str = "The clipboard could not be read; type the address, or try again";
const ALREADY: &str = "That link is already downloading; see Downloads";

impl PlayerApp {
    /// Add the address in the Search field to Downloads, and show the page.
    pub(super) fn download(&mut self) {
        let raw = String::from(self.ui.query.trim());
        let url = match nonos_download::parse(&raw) {
            Ok(url) => url,
            Err(why) => {
                self.trouble = Some(why);
                return;
            }
        };
        let shown = file_name(&url.target, "mp3");
        let name = shown.strip_suffix(".mp3").unwrap_or(&shown);
        match self.downloads.add(&raw, name) {
            Some(_) => {
                self.trouble = None;
                self.ui.query.clear();
                self.ui.go(View::Downloads);
                self.start_next();
            }
            None => self.trouble = Some(ALREADY),
        }
    }

    /// A Downloads row's button.
    pub(super) fn download_act(&mut self, id: u32, act: Act) {
        match act {
            Act::Cancel => {
                if self.downloads.cancel(id) {
                    fetch::cancel();
                }
            }
            Act::Resume => {
                if self.downloads.resume(id) {
                    self.start_next();
                }
            }
            Act::Play => {
                if let Some(State::Done(path)) = self.downloads.get(id).map(|r| r.state.clone()) {
                    self.play_path(&path);
                }
            }
            Act::Remove => {
                let partial = self.downloads.get(id).is_some_and(|r| !matches!(r.state, State::Done(_)));
                if self.downloads.remove(id) && partial {
                    let _ = vfs::unlink(mk_getpid(), part_path(id).as_bytes());
                }
            }
        }
    }

    /// Clear finished, the parts kept for a resume deleted with their rows.
    pub(super) fn clear_downloads(&mut self) {
        let pid = mk_getpid();
        for r in self.downloads.rows() {
            if matches!(r.state, State::Failed { .. } | State::Cancelled) {
                let _ = vfs::unlink(pid, part_path(r.id).as_bytes());
            }
        }
        self.downloads.clear_finished();
        self.ui.scroll = 0;
    }

    /// Start the next waiting download if the worker is free.
    fn start_next(&mut self) {
        if self.fetching.is_some() {
            return;
        }
        let Some(row) = self.downloads.start_next(mk_uptime_ms()) else { return };
        let (id, raw) = (row.id, row.url.clone());
        let started = nonos_download::parse(&raw).and_then(|url| fetch::start(url, part_path(id)));
        match started {
            Ok(()) => self.fetching = Some(id),
            Err(why) => self.downloads.ended(id, Err((why, fetch::transient(why)))),
        }
    }

    /// Ctrl+V on the Search page: the clipboard's first line onto the query.
    pub(super) fn paste(&mut self) {
        // Room for an address and more, so a longer one is seen as cut.
        let mut buf = [0u8; 2048];
        let said = match clipboard_paste_line(&mut buf) {
            Ok(Some(line)) => match paste_query(&mut self.ui.query, line.text) {
                Pasted::Whole => None,
                Pasted::Cut => Some(PASTE_CUT),
                Pasted::Empty => Some(PASTE_NONE),
            },
            Ok(None) => Some(PASTE_NONE),
            Err(_) => Some(PASTE_UNREACHED),
        };
        self.ui.scroll = 0;
        if said.is_some() {
            self.trouble = said;
        }
    }

    /// Follow the download a tick at a time. True when the window changed.
    pub(super) fn poll_fetch(&mut self) -> bool {
        let Some(id) = self.fetching else {
            return false;
        };
        let (_, done, total) = fetch::progress();
        self.downloads.progress(id, done, total, mk_uptime_ms());
        let Some(outcome) = fetch::outcome() else {
            return self.bar_moved(id);
        };
        self.fetching = None;
        self.downloads.ended(id, outcome.clone());
        self.fetch_line = String::new();
        if let Ok(path) = outcome {
            self.add_to_library(&path);
            if self.transport.state() != Play::Playing && self.loading.is_none() {
                self.play_path(&path);
            }
        }
        self.start_next();
        true
    }

    /// The bar's line for the running download; true when it changed.
    fn bar_moved(&mut self, id: u32) -> bool {
        let line = match self.downloads.get(id) {
            Some(r) => format!("Downloading {}: {}", r.name, row_text::status(r)),
            None => String::new(),
        };
        if line == self.fetch_line {
            return false;
        }
        self.fetch_line = line;
        true
    }

    /// A finished download joins the library, unless a rescan already
    /// listed it.
    fn add_to_library(&mut self, path: &str) {
        if self.library.tracks.iter().any(|t| same_path(&t.path, path)) {
            return;
        }
        self.library.tracks.push(Track::from_path(path));
        self.queue.enqueue(self.library.tracks.len() - 1);
    }

    /// Play the file at `path`: its library entry when it has one.
    fn play_path(&mut self, path: &str) {
        match self.library.tracks.iter().position(|t| same_path(&t.path, path)) {
            Some(i) => self.select(i),
            None => {
                self.outside = Some(Track::from_path(path));
                self.load_outside();
            }
        }
        self.ui.go(View::NowPlaying);
    }
}

/// Where row `id`'s bytes go until it is whole: one file per row and window,
/// so a resume finds what came and two windows never share one.
fn part_path(id: u32) -> String {
    format!("/tmp/music-{}-{id}.part", mk_getpid())
}
