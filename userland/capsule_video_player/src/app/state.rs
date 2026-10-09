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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_avi::AviFile;
use nonos_libc::{mk_getpid, mk_uptime_ms};

use super::browse::Browse;
use super::nav::Nav;
use crate::catalog::media::MediaItem;
use crate::player::{Clock, FrameDecoder, Source};
use crate::ui::screen::Route;

pub const WINDOW_ID: u32 = 0x5649;
pub const PATH: &str = "/video.avi";
pub const HEADER_MAX: u32 = 2 * 1024 * 1024;

pub struct VideoApp {
    pub(super) source: Option<Source>,
    pub(crate) file: Option<AviFile>,
    pub(super) decoder: FrameDecoder,
    pub(crate) clock: Clock,
    pub(super) frame: Vec<u32>,
    pub(super) cols: Vec<u32>,
    pub(super) rect: (u32, u32, u32, u32),
    pub(super) geom: (u32, u32),
    pub(crate) next: u32,
    pub(super) opened: bool,
    pub(crate) playing: bool,
    pub(crate) status: Option<&'static str>,
    pub(crate) dims: (u32, u32),
    pub(crate) force_decode: bool,
    pub(crate) nav: Nav,
    pub(crate) browse: Browse,
    pub(crate) path: String,
    /// A video handed over from outside the library's folders, probed so
    /// its details and position are its own and not the library selection's.
    pub(crate) outside: Option<MediaItem>,
    /// When next to ask the shell for a file to open (uptime, ms).
    pub(super) arg_due_ms: i64,
}

impl VideoApp {
    pub fn new() -> VideoApp {
        VideoApp {
            source: None,
            file: None,
            decoder: FrameDecoder::new(),
            clock: Clock::new(0, 0, 0),
            frame: Vec::new(),
            cols: Vec::new(),
            rect: (0, 0, 0, 0),
            geom: (0, 0),
            next: 0,
            opened: false,
            playing: false,
            status: None,
            dims: (1180, 760),
            force_decode: false,
            nav: Nav::new(Route::Library),
            browse: Browse::new(),
            path: String::from(PATH),
            outside: None,
            arg_due_ms: 0,
        }
    }

    pub(crate) fn route(&self) -> Route {
        self.nav.current()
    }

    pub(super) fn ensure_open(&mut self) {
        if self.opened {
            return;
        }
        self.opened = true;
        if let Err(e) = self.open() {
            self.status = Some(e);
            self.playing = false;
        }
    }

    fn open(&mut self) -> Result<(), &'static str> {
        let mut source = Source::open(mk_getpid(), self.path.as_bytes())?;
        let head = source.read_header(HEADER_MAX)?;
        // A film longer than the head keeps its index past it; read that
        // apart rather than refuse the file (`nonos_avi::AviFile::parse_parts`).
        let file = match AviFile::parse(&head) {
            Ok(file) => file,
            Err(_) => {
                let idx1 = source.read_index(&head)?;
                AviFile::parse_parts(&head, &idx1).map_err(|_| "not a playable avi")?
            }
        };
        let upf = file.header.micro_sec_per_frame;
        let start = self.start_frame(file.index.len() as u32, upf);
        let pixels = (file.video.width as usize)
            .checked_mul(file.video.height as usize)
            .ok_or("frame dimensions overflow")?;
        self.frame.try_reserve(pixels).map_err(|_| "frame too large")?;
        self.frame.resize(pixels, 0xff00_0000);
        // Opened where it was left in this window, if it was.
        self.clock = Clock::new(mk_uptime_ms(), Clock::new(0, 0, upf).pts_ms(start), upf);
        self.next = start;
        self.force_decode = true;
        self.source = Some(source);
        self.file = Some(file);
        Ok(())
    }
}
