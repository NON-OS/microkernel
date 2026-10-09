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
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{stat, VfsStream};
use nonos_libc::{mk_getpid, mk_time_millis};

use crate::decode::{self, Decoder};
use crate::library::Track;
use crate::loading::{Load, Source, Step, BARS};
use crate::model::TrackMeta;
use crate::track_fmt::format_of;
use crate::transport::Transport;
use crate::waveform::{from_samples, Waveform};

/// Time one tick may spend reading and decoding, so the window still paints
/// and takes clicks between ticks.
const STEP_MS: i64 = 8;

/// The file, a store round trip per chunk.
pub struct VfsSource(VfsStream);

impl Source for VfsSource {
    fn chunk(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str> {
        self.0.read_into(want, out)
    }
}

/// A track being loaded, carried from the app's ticks (`loading.rs`).
pub struct Loading {
    load: Load<VfsSource>,
    /// Play once loaded: a track the person chose, not the one shown at start.
    pub play: bool,
}

/// What a tick of loading came to.
pub enum Loaded {
    More,
    Ready(Waveform),
    Failed(&'static str),
}

/// An empty waveform, for a track not (yet) loaded.
pub fn blank() -> Waveform {
    from_samples(&[], BARS)
}

/// Begin loading `track`, or say why it cannot be. Either way the bar names
/// this track and the transport holds nothing of the one before it.
pub fn begin_track(
    transport: &mut Transport,
    meta: &mut TrackMeta,
    track: &Track,
    play: bool,
) -> Result<Loading, &'static str> {
    let load = begin(transport, meta, track.path.as_bytes(), &track.title, &track.artist)?;
    Ok(Loading { load, play })
}

fn begin(
    transport: &mut Transport,
    meta: &mut TrackMeta,
    path: &[u8],
    title: &str,
    artist: &str,
) -> Result<Load<VfsSource>, &'static str> {
    meta.title = String::from(title);
    meta.artist = String::from(artist);
    meta.format = String::new();
    // A failed load must not leave the last track's decoder behind, or play
    // would sound the old track under this one's title.
    transport.stop();
    let pid = mk_getpid();
    // A file past the limit is refused by its size before any of it is read
    // when the store can say it, else by a read longer than the limit.
    let size = stat(pid, path).ok().map(|(size, _)| size);
    let stream = VfsStream::open(pid, path)?;
    Load::new(VfsSource(stream), size, decode::open)
}

impl Loading {
    /// One tick's worth of the load. Once decoded through, the decoder goes
    /// to the transport and the waveform comes back.
    pub fn step(&mut self, transport: &mut Transport, meta: &mut TrackMeta) -> Loaded {
        let until = mk_time_millis().saturating_add(STEP_MS);
        match self.load.step(&mut || mk_time_millis() < until) {
            Step::More => Loaded::More,
            Step::Ready(dec, bars) => match finish(transport, meta, dec) {
                Ok(()) => Loaded::Ready(Waveform { buckets: bars }),
                Err(why) => Loaded::Failed(why),
            },
            Step::Failed(why) => Loaded::Failed(why),
        }
    }

    /// What the bar says while the track loads.
    pub fn progress(&self) -> Option<u32> {
        self.load.percent_read()
    }
}

fn finish(
    transport: &mut Transport,
    meta: &mut TrackMeta,
    dec: Box<dyn Decoder>,
) -> Result<(), &'static str> {
    meta.format = format_of(&*dec);
    transport.open(dec)
}
