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

//! Loading a track a tick at a time. A load reads the file whole (up to
//! 32 MiB) and then decodes all of it once for the waveform, and both ran
//! inside the click that chose the track: seconds of a frozen window for an
//! ordinary MP3, longer for a large one, with no sign anything was happening.
//!
//! Now each tick does as much as its budget allows and no more: a chunk of
//! the file per round trip to the store, a block of samples per decode, and
//! between ticks the window paints and takes clicks. The file is still held
//! once and the decoder still rewinds to play what it drew, as before.

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::decode::Decoder;
use crate::peaks::Peaks;
use crate::track_limit::{refuse_size, READ_LIMIT, TOO_LARGE};

/// Bars in a track's waveform.
pub const BARS: usize = 96;
/// Samples decoded per unit of work.
const BLOCK: usize = 4096;
/// Bytes asked for per unit of work: one store round trip.
pub const CHUNK: u32 = 64 * 1024;

/// Where the file's bytes come from, one round trip at a time.
pub trait Source {
    /// Append up to `want` bytes of the file to `out` and say how many came;
    /// 0 at the end.
    fn chunk(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str>;
}

/// Makes a decoder of the whole file.
pub type Open = fn(Vec<u8>) -> Result<Box<dyn Decoder>, &'static str>;

enum Phase<S> {
    Read(S),
    Decode(Box<dyn Decoder>, Peaks),
    Over,
}

/// What a step came to.
pub enum Step {
    /// Not done; step again on a later tick.
    More,
    /// Decoded through, rewound, and ready to play; with its waveform bars.
    Ready(Box<dyn Decoder>, Vec<u8>),
    Failed(&'static str),
}

pub struct Load<S> {
    phase: Phase<S>,
    bytes: Vec<u8>,
    size: Option<u64>,
    open: Open,
}

impl<S: Source> Load<S> {
    /// `size` is the file's length when the store said; a file past the
    /// limit is refused before any of it is read.
    pub fn new(source: S, size: Option<u64>, open: Open) -> Result<Self, &'static str> {
        if let Some(why) = size.and_then(refuse_size) {
            return Err(why);
        }
        let mut bytes = Vec::new();
        if let Some(n) = size {
            // Room for the whole file at once, so the read never doubles a
            // buffer the size of a track to grow it.
            let _ = bytes.try_reserve_exact(n as usize);
        }
        Ok(Load { phase: Phase::Read(source), bytes, size, open })
    }

    /// How far the load has come, in hundredths, while the file is read;
    /// None once it is decoding, or when the size is not known.
    pub fn percent_read(&self) -> Option<u32> {
        if !matches!(self.phase, Phase::Read(_)) {
            return None;
        }
        let size = self.size.filter(|&n| n > 0)?;
        Some(((self.bytes.len() as u64).saturating_mul(100) / size).min(100) as u32)
    }

    /// Do units of work while `more` says there is time, and at least one.
    pub fn step(&mut self, more: &mut dyn FnMut() -> bool) -> Step {
        loop {
            if let Some(done) = self.unit() {
                return done;
            }
            if !more() {
                return Step::More;
            }
        }
    }

    /// One chunk read or one block decoded; Some when the load is over.
    fn unit(&mut self) -> Option<Step> {
        match &mut self.phase {
            Phase::Read(source) => {
                // Never past one byte over the limit, as a whole-file read
                // asks: a file that long is refused, not cut short.
                let room = READ_LIMIT.saturating_sub(self.bytes.len() as u32);
                let n = match source.chunk(room.min(CHUNK), &mut self.bytes) {
                    Ok(n) => n,
                    Err(why) => return Some(self.fail(why)),
                };
                if refuse_size(self.bytes.len() as u64).is_some() {
                    return Some(self.fail(TOO_LARGE));
                }
                if n > 0 {
                    return None;
                }
                let bytes = core::mem::take(&mut self.bytes);
                match (self.open)(bytes) {
                    Ok(dec) => self.phase = Phase::Decode(dec, Peaks::default()),
                    Err(why) => return Some(self.fail(why)),
                }
                None
            }
            Phase::Decode(dec, peaks) => {
                let mut buf = [0i16; BLOCK];
                let n = dec.next(&mut buf);
                if n > 0 {
                    peaks.push(&buf[..n]);
                    return None;
                }
                let Phase::Decode(mut dec, peaks) =
                    core::mem::replace(&mut self.phase, Phase::Over)
                else {
                    return Some(Step::Failed("the load is over"));
                };
                if !dec.rewind() {
                    return Some(Step::Failed("track cannot be rewound"));
                }
                Some(Step::Ready(dec, peaks.finish(BARS)))
            }
            Phase::Over => Some(Step::Failed("the load is over")),
        }
    }

    fn fail(&mut self, why: &'static str) -> Step {
        self.phase = Phase::Over;
        self.bytes = Vec::new();
        Step::Failed(why)
    }
}
